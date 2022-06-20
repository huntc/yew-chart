use std::{
    ops::{Add, Sub},
    rc::Rc,
};

use chrono::{Duration, Utc};
use yew::prelude::*;
use yew_chart::{
    axis::{Axis, Orientation, Scale},
    linear_axis_scale::LinearScale,
    series::{self, Data, Labeller, Series, Type},
    time_axis_scale::TimeScale,
};

const WIDTH: f32 = 533.0;
const HEIGHT: f32 = 300.0;
const MARGIN: f32 = 50.0;
const TICK_LENGTH: f32 = 10.0;

struct App {
    data_set: Rc<Data>,
    vertical_axis_scale: Rc<dyn Scale>,
    horizontal_axis_scale: Rc<dyn Scale>,
    horizontal_series_scale: Rc<dyn Scale>,
}

impl Component for App {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let end_date = Utc::now();
        let start_date = end_date.sub(Duration::milliseconds(4));
        let time = start_date..end_date;
        let series_time = (start_date.timestamp_millis() & 0xffffff) as f32
            ..(end_date.timestamp_millis() & 0xffffff) as f32;

        let circle_labeller = Rc::from(series::circle_label()) as Rc<dyn Labeller>;
        let circle_text_labeller = Rc::from(series::circle_text_label("Label")) as Rc<dyn Labeller>;

        App {
            data_set: Rc::new(vec![
                ((start_date.timestamp_millis() & 0xffffff) as f32, 1.0, None),
                (
                    (start_date.add(Duration::milliseconds(1)).timestamp_millis() & 0xffffff)
                        as f32,
                    4.0,
                    Some(Rc::clone(&circle_labeller)),
                ),
                (
                    (start_date.add(Duration::milliseconds(2)).timestamp_millis() & 0xffffff)
                        as f32,
                    3.0,
                    Some(Rc::clone(&circle_labeller)),
                ),
                (
                    (start_date.add(Duration::milliseconds(3)).timestamp_millis() & 0xffffff)
                        as f32,
                    2.0,
                    Some(circle_labeller),
                ),
                (
                    (start_date.add(Duration::milliseconds(4)).timestamp_millis() & 0xffffff)
                        as f32,
                    5.0,
                    Some(circle_text_labeller),
                ),
            ]),
            horizontal_axis_scale: Rc::new(TimeScale::with_local_time_labeller(
                time,
                Duration::milliseconds(1),
                "%3f",
            )),
            horizontal_series_scale: Rc::new(LinearScale::new(
                series_time,
                Duration::milliseconds(1).num_milliseconds() as u32 as f32,
            )),
            vertical_axis_scale: Rc::new(LinearScale::new(0.0..5.0, 1.0)),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> yew::Html {
        html! {
            <svg class="chart" viewBox={format!("0 0 {} {}", WIDTH, HEIGHT)} preserveAspectRatio="none">
                <Series
                    series_type={Type::Scatter}
                    name="some-series"
                    data={Rc::clone(&self.data_set)}
                    horizontal_scale={Rc::clone(&self.horizontal_series_scale)}
                    vertical_scale={Rc::clone(&self.vertical_axis_scale)}
                    x={MARGIN} y={MARGIN} width={WIDTH - (MARGIN * 2.0)} height={HEIGHT - (MARGIN * 2.0)} />

                <Axis
                    name="some-y-axis"
                    orientation={Orientation::Left}
                    scale={Rc::clone(&self.vertical_axis_scale)}
                    x1={MARGIN} y1={MARGIN} xy2={HEIGHT - MARGIN}
                    tick_len={TICK_LENGTH}
                    title={"Some Y thing".to_string()} />

                <Axis
                    name="some-x-axis"
                    orientation={Orientation::Bottom}
                    scale={Rc::clone(&self.horizontal_axis_scale)}
                    x1={MARGIN} y1={HEIGHT - MARGIN} xy2={WIDTH - MARGIN}
                    tick_len={TICK_LENGTH}
                    title={"Some X thing (ms)".to_string()} />

            </svg>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
