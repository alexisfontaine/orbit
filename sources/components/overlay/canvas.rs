use leptos::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, Path2d};

use crate::components::State;


#[component]
pub fn Overlay (scope: Scope) -> Element {
	let state = use_context::<State>(scope).unwrap();
	let canvas = NodeRef::new(scope);

	let context = move || {
		let canvas: HtmlCanvasElement = canvas()?.unchecked_into();
		let context: CanvasRenderingContext2d = canvas.get_context("2d").ok()??.unchecked_into();

		context.set_fill_style(&"#a494".into());

		Some((context, canvas))
	};

	create_effect(scope, move |_| {
		let (context, canvas) = context()?;
		let (width, height) = state.size.get()?;
		let ratio = window().device_pixel_ratio();

		let (mut canvas_width, mut canvas_height) = (width, height);

		if ratio != 1. {
			canvas_width = (width * ratio).round();
			canvas_height = (height * ratio).round();

			let style = canvas.style();
			let _ = style.set_property("width", &format!("{}px", width));
			let _ = style.set_property("height", &format!("{}px", height));
			let _ = context.scale(width / canvas_width, height / canvas_height);
		}

		canvas.set_width(canvas_width as _);
		canvas.set_height(canvas_height as _);
		Some(())
	});

	create_effect(scope, move |_| {
		let (context, _) = context()?;
		let (width, height) = state.size.get()?;
		let viewport = &state.viewport();
		let matrix = &viewport.matrix;

		context.clear_rect(0., 0., width, height);

		for style in &state.camera_untracked().styles {
			let path = state.scene.shapes[style.index].path(width, height, matrix)?;

			context.fill_with_path_2d(&Path2d::new_with_path_string(&path).ok()?);
		}

		Some(())
	});

	view!(scope,
		<canvas _ref=canvas class="canvas" />
	)
}
