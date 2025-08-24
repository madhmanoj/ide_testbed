use std::rc::Rc;

use dominator::{clone, events, html, Dom, DomBuilder, EventOptions};
use futures::{channel::mpsc, StreamExt};
use futures_signals::{map_ref, signal::{LocalBoxSignal, Mutable, SignalExt}, signal_vec::{MutableVec, SignalVecExt}};
use itertools::Either;
use web_sys::HtmlElement;
use crate::{styles, workspace::activity_panel::{editor, Activity, ActivityPanelCommand}};

use super::{activity_panel::ActivityPanel, Workspace};

const RESIZER_PX: f64 = crate::RESIZER_PX as f64;
const MIN_PANEL_SIZE: f64 = 100.0;

pub struct GridAttributes {
    pub column_start: usize,
    pub column_end: usize,
    pub row_start: usize,
    pub row_end: usize,
}

impl GridAttributes {
    fn apply(&self, builder: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
        builder
            .style("grid-column-start", self.column_start.to_string())
            .style("grid-column-end", self.column_end.to_string())
            .style("grid-row-start", self.row_start.to_string())
            .style("grid-row-end", self.row_end.to_string())
    }
}

pub enum LayoutPanel {
    HorizontalSplit {
        // | | |
        parent: Option<Rc<LayoutPanel>>,
        children: MutableVec<(Rc<LayoutPanel>, Mutable<f64>)>,
    },
    VerticalSplit {
        // _
        // _
        // _
        parent: Option<Rc<LayoutPanel>>,
        children: MutableVec<(Rc<LayoutPanel>, Mutable<f64>)>,
    },
    Widget {
        parent: Option<Rc<LayoutPanel>>,
        activity_panel: Rc<ActivityPanel>,
    },
}

async fn redistribute((available_space, element_sizes): (f64, Vec<Mutable<f64>>)) {
    match element_sizes.iter().len() {
        0 => (),
        element_count => {
            // to make space for resizer and prevent overflow
            let available_space = available_space - ((element_sizes.len() as f64 - 1.0) * RESIZER_PX);
            let mut element_sizes = element_sizes.iter()
                .map(Mutable::lock_mut)
                .collect::<Vec<_>>();
            /* calculate the total size */
            let current_total_size = element_sizes.iter()
                .fold(0.0, |total, size| total + **size);
            /* handle current_total_size == 0 */
            if current_total_size == 0.0 {
                let element_size = available_space / element_count as f64;
                element_sizes.iter_mut().for_each(|size| **size = element_size);
            }
            else {
                let scaling_factor = available_space /
                    current_total_size;
                element_sizes.iter_mut().for_each(|size| **size *= scaling_factor);
            }
        }
    }
}

fn horizontal_resizer(
    left_index: usize,
    children: &MutableVec<(Rc<LayoutPanel>, Mutable<f64>)>,
    grid_attributes: GridAttributes,
) -> Dom {
    let resizer_last_position = Mutable::new(None);
    let resizer_hover = Mutable::new(false);
    
    html!("div", {
        .apply(|builder| grid_attributes.apply(builder))
        .style("width", format!("{RESIZER_PX}px"))
        .style("background-color", "white")
        .style("cursor", "ew-resize")
        .style_signal("background-color", resizer_hover.signal_ref(|hover| {
            if *hover {
                styles::FEATURE_COLOR
            } else {
                styles::BACKGROUND_COLOR
            }
        }))
        .event_with_options(
            &EventOptions::preventable(),
            clone!(resizer_last_position => move |ev: events::PointerDown| {
                resizer_last_position.set(Some(ev.x()));
                ev.prevent_default();
            })
        )
        .global_event(clone!(resizer_last_position => move |_: events::PointerUp| {
            resizer_last_position.set(None);
        }))
        .event(clone!(resizer_hover => move |_: events::PointerEnter| {
            resizer_hover.set_neq(true);
        }))
        .event(clone!(resizer_hover => move |_: events::PointerLeave| {
            resizer_hover.set_neq(false);
        }))
        .global_event(clone!(resizer_last_position, children => move |ev: events::PointerMove| {
            if let Some(last_position) = resizer_last_position.get() {
                let required_delta = (ev.x() - last_position) as f64;
                resizer_last_position.set(Some(ev.x()));
                let children = children.lock_ref();
                if required_delta < 0.0 {
                    let mut acquired_delta = 0.0;
                    // left_index + 1 is always valid because resizers are only created between valid adjacent LayoutPanels
                    let (_, right_panel_width) = children.get(left_index + 1).unwrap();
                    let children = children.get(..left_index + 1).into_iter().flat_map(|slice| slice.iter()).rev();
                    let mut right_panel_width = right_panel_width.lock_mut();
                    for (_, child_size) in children {
                        let mut child_size = child_size.lock_mut();
                        let available_space = (*child_size - MIN_PANEL_SIZE)
                            .clamp(0.0, (required_delta - acquired_delta).abs());
                        *right_panel_width += available_space;
                        *child_size -= available_space;
                        acquired_delta -= available_space;
                    }
                }
                else {
                    let mut acquired_delta = 0.0;
                    // left_index + 1 is always valid because resizers are only created between valid adjacent LayoutPanels
                    let (_, left_panel_width) = children.get(left_index).unwrap();
                    let children = children.get(left_index + 1..).into_iter().flat_map(|slice| slice.iter());
                    let mut left_panel_width = left_panel_width.lock_mut();
                    for (_, child_size) in children {
                        let mut child_size = child_size.lock_mut();
                        let available_space = (*child_size - MIN_PANEL_SIZE)
                            .clamp(0.0, (required_delta - acquired_delta).abs());
                        *left_panel_width += available_space;
                        *child_size -= available_space;
                        acquired_delta += available_space;
                    }
                }
            }
        }))
    })
}

fn vertical_resizer(
    top_index: usize,
    children: &MutableVec<(Rc<LayoutPanel>, Mutable<f64>)>,
    grid_attributes: GridAttributes,
) -> Dom {
    let resizer_last_position = Mutable::new(None);
    let resizer_hover = Mutable::new(false);
    
    html!("div", {
        .apply(|builder| grid_attributes.apply(builder))
        .style("height", format!("{RESIZER_PX}px"))
        .style("cursor", "ns-resize")
        .style_signal("background-color", resizer_hover.signal_ref(|hover| {
            if *hover {
                styles::FEATURE_COLOR
            } else {
                styles::BACKGROUND_COLOR
            }
        }))
        .event_with_options(
            &EventOptions::preventable(),
            clone!(resizer_last_position => move |ev: events::PointerDown| {
                resizer_last_position.set(Some(ev.y()));
                ev.prevent_default();
            })
        )
        .global_event(clone!(resizer_last_position => move |_: events::PointerUp| {
            resizer_last_position.set(None);
        }))
        .event(clone!(resizer_hover => move |_: events::PointerEnter| {
            resizer_hover.set_neq(true);
        }))
        .event(clone!(resizer_hover => move |_: events::PointerLeave| {
            resizer_hover.set_neq(false);
        }))
        .global_event(clone!(resizer_last_position, children => move |ev: events::PointerMove| {
            if let Some(last_position) = resizer_last_position.get() {
                let required_delta = (ev.y() - last_position) as f64;
                resizer_last_position.set(Some(ev.y()));
                let children = children.lock_ref();
                if required_delta < 0.0 {
                    let mut acquired_delta = 0.0;
                    // top_index + 1 is always valid because resizers are only created between valid adjacent LayoutPanels
                    let (_, bottom_panel_width) = children.get(top_index + 1).unwrap();
                    let children = children.get(..top_index + 1).into_iter().flat_map(|slice| slice.iter()).rev();
                    let mut bottom_panel_width = bottom_panel_width.lock_mut();
                    for (_, child_size) in children {
                        let mut child_size = child_size.lock_mut();
                        let available_space = (*child_size - MIN_PANEL_SIZE)
                            .clamp(0.0, (required_delta - acquired_delta).abs());
                        *bottom_panel_width += available_space;
                        *child_size -= available_space;
                        acquired_delta -= available_space;
                    }
                }
                else {
                    let mut acquired_delta = 0.0;
                    // top_index + 1 is always valid because resizers are only created between valid adjacent LayoutPanels
                    let (_, top_panel_width) = children.get(top_index).unwrap();
                    let children = children.get(top_index + 1..).into_iter().flat_map(|slice| slice.iter());
                    let mut top_panel_width = top_panel_width.lock_mut();
                    for (_, child_size) in children {
                        let mut child_size = child_size.lock_mut();
                        let available_space = (*child_size - MIN_PANEL_SIZE)
                            .clamp(0.0, (required_delta - acquired_delta).abs());
                        *top_panel_width += available_space;
                        *child_size -= available_space;
                        acquired_delta += available_space;
                    }
                }
            }
        }))
    })
}

impl LayoutPanel {
    pub fn render(
        workspace: &Rc<Workspace>,
        this: &Rc<LayoutPanel>,
        grid_attributes: GridAttributes,
        width: LocalBoxSignal<'static, f64>,
        height: LocalBoxSignal<'static, f64>,
    ) -> Dom {       
        html!("div", {
            .apply(|builder| grid_attributes.apply(builder))
            .apply(move |builder| {
                match this.as_ref() {
                    LayoutPanel::Widget { activity_panel, .. } => {
                        let height = height.broadcast();
                        let width = width.broadcast();

                        let (activity_panel_tx, activity_panel_rx) = mpsc::unbounded::<ActivityPanelCommand>();

                        activity_panel.activity_panel_tx.set(Some(activity_panel_tx));
                        builder
                            .style_signal("height", height.signal().map(|h| format!("{h}px")))
                            .style_signal("width", width.signal().map(|w| format!("{w}px")))
                            // to set active LayoutPanel for opening files
                            .event(clone!(workspace, activity_panel => move |_: events::Click| {
                                workspace.as_ref().active_panel.set(activity_panel.clone());
                            }))
                            // to get rid of unbounded sender on closing activity LayoutPanels
                            .future(activity_panel_rx.for_each(clone!(activity_panel => move |command| clone!(activity_panel => async move {
                                match command {
                                    ActivityPanelCommand::OpenFile(file) => {
                                        let mut activities = activity_panel.activities.lock_mut();
                                        let editor = activities.iter()
                                            .find(|activity| match &***activity {
                                                Activity::Editor(editor) => Rc::ptr_eq(&editor.file, &file),
                                                _ => false,
                                            })
                                            .cloned()
                                            .unwrap_or_else(move || {
                                                let editor = Rc::new(Activity::Editor(Rc::new(editor::Editor::new(file))));
                                                activities.push_cloned(editor.clone());
                                                editor
                                            });
                                        activity_panel.active_activity.set(Some(editor));
                                    },
                                }
                            }))))
                            .child(ActivityPanel::render(this, activity_panel, height.signal()))
                    },
                    LayoutPanel::HorizontalSplit { children, .. } => {
                        let height = height.broadcast();
                        let width = width.broadcast();

                        let grid_template_columns = children
                            .signal_vec_cloned()
                            .len()
                            .map(|len| match len {
                                0 => String::from("none"),
                                len => format!("repeat({len}, auto)")
                            });

                        let redistribute_width = map_ref!{
                            let width = width.signal(),
                            let children_widths = children
                                .signal_vec_cloned()
                                .map(|(_, width)| width)
                                .to_signal_cloned() => {
                                    (*width, children_widths.clone())
                                }
                        }.for_each(redistribute);

                        let horizontal_children = children.signal_vec_cloned().to_signal_map(clone!(children, workspace => move |nodes| {
                            let mut resizer_index = 0;
                            let resizer = move || {
                                let resizer = Either::Right(resizer_index);
                                resizer_index += 1;
                                resizer
                            };
                            itertools::intersperse_with(nodes.iter().map(Either::Left), resizer)
                                .enumerate()
                                .map(clone!(children, height, workspace => move |(node_index, node)| match node {
                                    Either::Left((panel, panel_width)) => LayoutPanel::render(
                                        &workspace,
                                        panel,
                                        GridAttributes {
                                            column_start: 1 + node_index,
                                            column_end: 2 + node_index,
                                            row_start: 1,
                                            row_end: 2
                                        },
                                        panel_width.signal().boxed_local(),
                                        height.signal().boxed_local()
                                    ),
                                    Either::Right(left_index) => horizontal_resizer(
                                        left_index,
                                        &children,
                                        GridAttributes {
                                            column_start: 1 + node_index,
                                            column_end: 2 + node_index,
                                            row_start: 1,
                                            row_end: 2
                                        }
                                    ),
                                }))
                                .collect::<Vec<_>>()
                        })).to_signal_vec();

                        builder
                            .future(redistribute_width)
                            .style("display", "grid")
                            .style_signal("grid-template-columns", grid_template_columns)
                            .style("grid-template-rows", "auto")
                            .children_signal_vec(horizontal_children)
                    },
                    LayoutPanel::VerticalSplit { children, .. } => {
                        let height = height.broadcast();
                        let width = width.broadcast();

                        let grid_template_rows = children
                            .signal_vec_cloned()
                            .len()
                            .map(|len| match len {
                                0 => String::from("none"),
                                len => format!("repeat({len}, auto)")
                            });

                        let redistribute_height = map_ref!{
                            let height = height.signal(),
                            let children_heights = children
                                .signal_vec_cloned()
                                .map(|(_, height)| height)
                                .to_signal_cloned() => {
                                    (*height, children_heights.clone())
                                }
                        }.for_each(redistribute);

                        let vertical_children = children.signal_vec_cloned().to_signal_map(clone!(children, workspace => move |nodes| {
                            let mut resizer_index = 0;
                            let resizer = move || {
                                let resizer = Either::Right(resizer_index);
                                resizer_index += 1;
                                resizer
                            };
                            itertools::intersperse_with(nodes.iter().map(Either::Left), resizer)
                                .enumerate()
                                .map(clone!(children, width, workspace => move |(node_index, node)| match node {
                                    Either::Left((panel, panel_height)) => LayoutPanel::render(
                                        &workspace,
                                        panel,
                                        GridAttributes {
                                            column_start: 1,
                                            column_end: 2,
                                            row_start: 1 + node_index,
                                            row_end: 2 + node_index
                                        },
                                        width.signal().boxed_local(),
                                        panel_height.signal().boxed_local()
                                    ),
                                    Either::Right(top_index) => vertical_resizer(
                                        top_index,
                                        &children,
                                        GridAttributes {
                                            column_start: 1,
                                            column_end: 2,
                                            row_start: 1 + node_index,
                                            row_end: 2 + node_index
                                        }
                                    ),
                                }))
                                .collect::<Vec<_>>()
                        })).to_signal_vec();

                        builder
                            .future(redistribute_height)
                            .style("display", "grid")
                            .style("grid-template-columns", "auto")
                            .style_signal("grid-template-rows", grid_template_rows)
                            .children_signal_vec(vertical_children)
                    }
                } 
            })
        })
    }

    pub fn append_widget(self: &Rc<LayoutPanel>, size: f64, activity_panel: Rc<ActivityPanel>) -> Rc<LayoutPanel> {
        match self.as_ref() {
            LayoutPanel::HorizontalSplit { children, .. } | LayoutPanel::VerticalSplit { children, .. } => {
                let split = Rc::new(LayoutPanel::Widget {
                    parent: Some(self.clone()),
                    activity_panel
                });
                children.lock_mut().push_cloned((split.clone(), Mutable::new(size)));
                split
            }
            LayoutPanel::Widget { .. } => panic!("Cannot append a widget to a widget"),
        }
    }

    pub fn append_split_horizontal(self: &Rc<LayoutPanel>, size: f64) -> Rc<LayoutPanel> {
        match self.as_ref() {
            LayoutPanel::HorizontalSplit { .. } => self.clone(),
            LayoutPanel::VerticalSplit { children, .. } => {
                let split = Rc::new(LayoutPanel::HorizontalSplit {
                    parent: Some(self.clone()),
                    children: MutableVec::default(),
                });
                children.lock_mut().push_cloned((split.clone(), Mutable::new(size)));
                split
            }
            LayoutPanel::Widget { .. } => panic!("Cannot append a horizontal split to a widget"),
        }
    }

    pub fn append_split_vertical(self: &Rc<LayoutPanel>, size: f64) -> Rc<LayoutPanel> {
        match self.as_ref() {
            LayoutPanel::HorizontalSplit { children, .. } => {
                let split = Rc::new(LayoutPanel::VerticalSplit {
                    parent: Some(self.clone()),
                    children: MutableVec::default(),
                });
                children.lock_mut().push_cloned((split.clone(), Mutable::new(size)));
                split
            },
            LayoutPanel::VerticalSplit { .. } => self.clone(),
            LayoutPanel::Widget { .. } => panic!("Cannot append a vertical split to a widget"),
        }
    }

    pub fn split_horizontal(self: &Rc<LayoutPanel>, split_left: bool) {
        if let LayoutPanel::Widget { parent: Some(parent), activity_panel } = self.as_ref() {
            match parent.as_ref() {
                LayoutPanel::HorizontalSplit { children, .. } => {
                    // index should always be a Some value, since self should always be present in children mutablevec of its parent
                    // otherwise panic since the LayoutPanel is invalid
                    let index = children.lock_ref().iter().position(|(child, _)| {
                        Rc::ptr_eq(child, self)
                    }).unwrap();

                    let mut children = children.lock_mut();
                    let (total_size, count) = children.iter()
                        .fold((0.0, 0.0), |(total_size, count), (_, size)| (total_size + size.get(), count + 1.0));
                    let new_size = total_size / count;

                    let active_activity = activity_panel.active_activity.get_cloned();
                    if let Some(active_activity) = active_activity {
                        let new_activity_panel = ActivityPanel::new(&active_activity);
                        if split_left {
                            let new_panel = Rc::new(LayoutPanel::Widget { parent: Some(parent.clone()), activity_panel: new_activity_panel.clone() });
                            children.insert_cloned(index, (new_panel, Mutable::new(new_size)));
                        } else {
                            let new_panel = Rc::new(LayoutPanel::Widget { parent: Some(parent.clone()), activity_panel: new_activity_panel.clone() });
                            children.insert_cloned(index + 1, (new_panel, Mutable::new(new_size)));
                        }
                    }
                },
                LayoutPanel::VerticalSplit { children, .. } => {
                    // index should always be a Some value, since self should always be present in children mutablevec of its parent
                    // otherwise panic since the LayoutPanel is invalid
                    let index = children.lock_ref().iter().position(|(child, _)| {
                        Rc::ptr_eq(child, self)
                    }).unwrap();

                    let mut lock = children.lock_mut();
                    // if we have a valid index from the previous unwrap, there should be a corresponding size for that
                    let old_size = lock.get(index).unwrap().1.get();

                    let active_activity = activity_panel.active_activity.get_cloned();
                    if let Some(active_activity) = active_activity {
                        let new_activity_panel = ActivityPanel::new(&active_activity);

                        if split_left {
                            let split_horizontal = Rc::new(LayoutPanel::HorizontalSplit { parent: Some(parent.clone()), children: MutableVec::default() });
                            split_horizontal.append_widget(100.0, new_activity_panel.clone());
                            split_horizontal.append_widget(100.0, activity_panel.clone());

                            lock.set_cloned(index, (split_horizontal, Mutable::new(old_size)));
                        } else {
                            let split_horizontal = Rc::new(LayoutPanel::HorizontalSplit { parent: Some(parent.clone()), children: MutableVec::default() });
                            split_horizontal.append_widget(100.0, activity_panel.clone());
                            split_horizontal.append_widget(100.0, new_activity_panel.clone());

                            lock.set_cloned(index, (split_horizontal, Mutable::new(old_size)));
                        }
                    }
                },
                LayoutPanel::Widget { .. } => unreachable!("A widget cannot be a parent")
            }
        }
    }

    // See notes in split horizontal
    pub fn split_vertical(self: &Rc<LayoutPanel>, split_up: bool) {
        if let LayoutPanel::Widget { parent: Some(parent), activity_panel } = self.as_ref() {
            match parent.as_ref() {
                LayoutPanel::HorizontalSplit { children, .. } => {
                    // index should always be a Some value, since self should always be present in children mutablevec of its parent
                    // otherwise panic since the LayoutPanel is invalid
                    let index = children.lock_ref().iter().position(|(child, _)| {
                        Rc::ptr_eq(child, self)
                    }).unwrap();

                    let mut lock = children.lock_mut();
                    // if we have a valid index from the previous unwrap, there should be a corresponding size for that
                    let old_size = lock.get(index).unwrap().1.get();

                    let active_activity = activity_panel.active_activity.get_cloned();
                    if let Some(active_activity) = active_activity {
                        let new_activity_panel = ActivityPanel::new(&active_activity);

                        if split_up {
                            let split_horizontal = Rc::new(LayoutPanel::VerticalSplit { parent: Some(parent.clone()), children: MutableVec::default() });
                            split_horizontal.append_widget(100.0, new_activity_panel.clone());
                            split_horizontal.append_widget(100.0, activity_panel.clone());

                            lock.set_cloned(index, (split_horizontal, Mutable::new(old_size)));
                        } else {
                            let split_horizontal = Rc::new(LayoutPanel::VerticalSplit { parent: Some(parent.clone()), children: MutableVec::default() });
                            split_horizontal.append_widget(100.0, activity_panel.clone());
                            split_horizontal.append_widget(100.0, new_activity_panel.clone());

                            lock.set_cloned(index, (split_horizontal, Mutable::new(old_size)));
                        }
                    }
                },
                LayoutPanel::VerticalSplit { children, .. } => {
                    // index should always be a Some value, since self should always be present in children mutablevec of its parent
                    // otherwise panic since the LayoutPanel is invalid
                    let index = children.lock_ref().iter().position(|(child, _)| {
                        Rc::ptr_eq(child, self)
                    }).unwrap();

                    let mut children = children.lock_mut();
                    let (total_size, count) = children.iter()
                        .fold((0.0, 0.0), |(total_size, count), (_, size)| (total_size + size.get(), count + 1.0));
                    let new_size = total_size / count;

                    let active_activity = activity_panel.active_activity.get_cloned();
                    if let Some(active_activity) = active_activity {
                        let new_activity_panel = ActivityPanel::new(&active_activity);
                        if split_up {
                            let new_panel = Rc::new(LayoutPanel::Widget { parent: Some(parent.clone()), activity_panel: new_activity_panel.clone() });
                            children.insert_cloned(index, (new_panel, Mutable::new(new_size)));
                        } else {
                            let new_panel = Rc::new(LayoutPanel::Widget { parent: Some(parent.clone()), activity_panel: new_activity_panel.clone() });
                            children.insert_cloned(index + 1, (new_panel, Mutable::new(new_size)));
                        }
                    }
                },
                LayoutPanel::Widget { .. } => unreachable!("A widget cannot be a parent")
            }
        }
    }
}