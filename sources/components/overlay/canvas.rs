use leptos::*;
use wasm_bindgen::intern;
use web_sys::{CanvasRenderingContext2d, Path2d};

use crate::model::Style;
use crate::state::use_state;


#[component]
pub fn Overlay (scope: Scope) -> impl IntoView {
	let canvas = NodeRef::<HtmlElement<Canvas>>::new(scope);
	let state = use_state(scope);

	let context = move || {
		let canvas = canvas()?;
		let context: CanvasRenderingContext2d = canvas.get_context(intern("2d").ok()??.unchecked_into();

		context.set_fill_style(&"#a494".into());

		Some((context, canvas))
	};

	create_effect(scope, move |_| {
		let (context, canvas) = context()?;
		let (width, height) = state.get_size()?;
		let ratio = window().device_pixel_ratio();

		let (mut canvas_width, mut canvas_height) = (width, height);

		if (ratio - 1.).abs() < f64::EPSILON {
			canvas_width = (width * ratio).round();
			canvas_height = (height * ratio).round();

			let style = canvas.style();
			let _ = style.set_property(intern("width"), &format!("{width}px"));
			let _ = style.set_property(intern("height"), &format!("{height}px"));
			let _ = context.scale(width / canvas_width, height / canvas_height);
		}

		canvas.set_width(canvas_width as _);
		canvas.set_height(canvas_height as _);
		Some(())
	});

	create_effect(scope, move |_| {
		let (context, _) = context()?;
		let (width, height) = state.get_size()?;

		context.clear_rect(0., 0., width, height);

		let scene = state.get_scene();
		let shapes = &scene.shapes;
		let camera = &scene.cameras[state.get_camera()];
		let viewport = &camera.viewports[state.get_viewport()];

		draw(&camera.styles, &move |style| {
			let style = style.get_shape()?;

			if let Some(path) = shapes[style.index].path(width, height, &viewport.matrix, style.back_face_culling.then_some(viewport.position), style) {
				context.fill_with_path_2d(&Path2d::new_with_path_string(&path).ok()?);
			}

			Some(())
		})
	});

	view!(scope,
		<canvas _ref=canvas class="overlay" />
	)
}


fn draw (styles: &[Style], with: &impl Fn(&Style) -> Option<()>) -> Option<()> {
	for style in styles {
		if let Some(style) = style.get_compound() {
			draw(&style.children, with);
		} else {
			with(style)?;
		}
	}

	Some(())
}
