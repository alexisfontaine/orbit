use leptos::*;

use crate::components::State;


#[component]
fn Path (scope: Scope, width: f64, height: Signal<f64>, index: usize, class: String) -> impl IntoView {
	let state = use_context::<State>(scope).unwrap();
	let viewport = state.viewport();

	view!(scope,
		<path
			class=format!("shape {}", class)
			d=move || viewport.with(|viewport| state.scene.shapes[index]
				.path(width, height(), &viewport.matrix))
		/>
	)
}

#[component]
pub fn Overlay (scope: Scope) -> impl IntoView {
	const WIDTH: f64 = 1_000.;


	let state = use_context::<State>(scope).unwrap();
	let camera = state.camera();

	let height = Signal::derive(scope, move || camera.with(|camera| {
		let value = camera.aspect_ratio.split_once('/').unwrap();

		(WIDTH / value.0.parse::<f64>().unwrap() * value.1.parse::<f64>().unwrap()).ceil()
	}));

	view!(scope,
		<svg
			class="overlay"
			viewBox=move || format!("0 0 {:.0} {:.0}", WIDTH, height())
			xmlns="http://www.w3.org/2000/svg"
		>
			// FIXME: Make use of the `For` component
			{move || camera.with(|camera| camera.styles
				.iter()
				.map(|style| view!(scope,
					<Path 
						class=style.name.clone()
						height=height.into()
						index=style.index
						width=WIDTH
					/>
				))
				.collect::<Vec<_>>())}
		</svg>
	)
}
