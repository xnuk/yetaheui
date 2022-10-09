use gloo_timers::callback::Interval;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{console, window, HtmlElement, HtmlTextAreaElement};
use yetaheui::parse::CodeMatrix;
use yew::functional::{
	function_component, use_node_ref, use_state, use_state_eq,
};
use yew::{
	self, html, start_app, Callback, Component, Context, MouseEvent, NodeRef,
	Properties,
};

use yetaheui::{
	parse::parse,
	runner::{AheuiIO, AheuiNum, Runner},
};

struct Fuck {
	output_ref: NodeRef,
}

impl AheuiIO for Fuck {
	type Num = AheuiNum;

	fn get_num(&mut self) -> Option<Self::Num> {
		window()?
			.prompt_with_message("숫자를 입력하세요")
			.ok()
			.flatten()
			.and_then(|x| x.trim().parse().ok())
	}

	fn get_char(&mut self) -> Option<char> {
		window()?
			.prompt_with_message("문자를 입력하세요")
			.ok()
			.flatten()
			.and_then(|x| x.chars().next())
	}

	fn put_num(&mut self, num: Self::Num) {
		if let Some(el) = self.output_ref.cast::<HtmlElement>() {
			let mut text = el.text_content().unwrap_or_default();
			text.push_str(&format!("{num}"));
			el.set_text_content(Some(&text));
		}
	}

	fn put_char(&mut self, ch: char) {
		if let Some(el) = self.output_ref.cast::<HtmlElement>() {
			let mut text = el.text_content().unwrap_or_default();
			text.push(ch);
			el.set_text_content(Some(&text));
		}
	}
}

#[derive(PartialEq, Properties, Default)]
struct Count {
	value: i32,
}

#[derive(PartialEq, Eq, Default)]
enum RunState {
	#[default]
	Initial,
	Running,
	Paused,
}

#[derive(PartialEq, Properties, Default)]
struct Pr {
	running_state: RunState,
}

fn get_text(text_ref: &NodeRef) -> String {
	text_ref
		.cast::<HtmlTextAreaElement>()
		.map(|v| v.value())
		.unwrap_or_default()
}

#[derive(Default)]
struct MyMain {
	runner: Option<Runner<CodeMatrix, Fuck>>,
	text_ref: NodeRef,
	output_ref: NodeRef,
	running_state: RunState,
	interval: Option<Interval>,
}

impl MyMain {
	fn create_runner(&mut self) {
		let code = parse(&get_text(&self.text_ref));
		let fio = Fuck {
			output_ref: self.output_ref.clone(),
		};
		self.runner = Some(Runner::new(code, fio));
		self.running_state = RunState::Paused;
	}

	fn step_runner(&mut self) {
		self.clear_interval();
		if let Some(runner) = &mut self.runner {
			runner.step();
			self.running_state = RunState::Paused;
		}
	}

	fn destroy_runner(&mut self) {
		self.clear_interval();
		self.runner = None;
		self.running_state = RunState::Initial;
	}

	fn runaway(&mut self) {
		if let Some(runner) = &mut self.runner {
			self.running_state = RunState::Running;
			for _ in 0..40 {
				if runner.step().is_some() {
					return;
				};
			}
		}
	}

	fn clear_interval(&mut self) {
		if let Some(interval) = self.interval.take() {
			interval.cancel();
		}
		self.running_state = RunState::Paused;
	}
}

enum Msg {
	CreateRunner,
	StepRunner,
	DestroyRunner,
	Runaway,
	Pause,
	Void,
}

impl Component for MyMain {
	type Message = Msg;
	type Properties = ();

	fn create(ctx: &Context<Self>) -> Self {
		Default::default()
	}

	fn view(&self, ctx: &Context<Self>) -> yew::Html {
		let create_runner = ctx.link().callback(|_| Msg::CreateRunner);
		let step = ctx.link().callback(|_| Msg::StepRunner);
		let destroy = ctx.link().callback(|_| Msg::DestroyRunner);
		let runaway = ctx.link().callback(|_| Msg::Runaway);
		let pause = ctx.link().callback(|_| Msg::Pause);

		let buttons = match self.running_state {
			RunState::Initial => html! {
				<button onclick={create_runner}>{"다음"}</button>
			},
			RunState::Running => html! {
				<>
					<button onclick={step}>{"다음"}</button>
					<button onclick={destroy}>{"초기화"}</button>
					<button onclick={pause}>{"멈춤"}</button>
				</>
			},
			RunState::Paused => html! {
				<>
					<button onclick={step}>{"다음"}</button>
					<button onclick={destroy}>{"초기화"}</button>
					<button onclick={runaway}>{"실행"}</button>
				</>
			},
		};

		html! {
			<div>
				<textarea
					ref={self.text_ref.clone()}
					placeholder={"밯망희"}
					style={"width: 80ch; height: 60ch"}
				></textarea>
				<div>{buttons}</div>
				{
					if self.running_state != RunState::Initial {
						html!{
							<pre ref={self.output_ref.clone()}></pre>
						}
					} else { html!{} }
				}
			</div>
		}
	}

	fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
		match msg {
			Msg::CreateRunner => {
				self.create_runner();
				true
			}
			Msg::StepRunner => {
				self.step_runner();
				true
			}
			Msg::DestroyRunner => {
				self.destroy_runner();
				true
			}
			Msg::Runaway => {
				self.runaway();
				let cb = ctx.link().callback(|_| Msg::Runaway);
				self.interval = Some(Interval::new(0, move || cb.emit(())));
				true
			}
			Msg::Pause => {
				self.clear_interval();
				true
			}
			_ => false,
		}
	}

	fn destroy(&mut self, _ctx: &Context<Self>) {
		self.clear_interval();
	}
}

#[function_component(Counter)]
fn counter(props: &Count) -> Html {
	let count = use_state_eq(|| props.value);

	let add_one = {
		let count = count.clone();
		Callback::from(move |_| count.set(*count + 1))
	};

	let sub_one = {
		let count = count.clone();
		Callback::from(move |event: MouseEvent| {
			count.set(*count - 1);
			event.stop_propagation();
			event.prevent_default();
		})
	};

	html! {
		<div>
			<span>{ *count }</span>
			<button onclick={add_one} oncontextmenu={sub_one}>{"+1"}</button>
		</div>
	}
}

fn main() {
	start_app::<MyMain>();
}
