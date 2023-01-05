use leptos::*;
use web_sys::{CanvasRenderingContext2d, Path2d};

use crate::components::State;


#[component]
pub fn Overlay (scope: Scope) -> impl IntoView {
	let canvas = NodeRef::<HtmlElement<Canvas>>::new(scope);
	let state = use_context::<State>(scope).unwrap();
	let viewport = state.viewport();

	let context = move || {
		let canvas = canvas()?;
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

		context.clear_rect(0., 0., width, height);

		viewport.with(|viewport| {
			for style in &state.camera_untracked().styles {
				let path = state.scene.shapes[style.index].path(width, height, &viewport.matrix)?;

				context.fill_with_path_2d(&Path2d::new_with_path_string(&path).ok()?);
			}

			Some(())
		})
	});

	view!(scope,
		<canvas _ref=canvas class="overlay" />
	)
}
