use leptos::*;

use crate::components::State;


#[component]
fn Path (scope: Scope, width: f64, height: Signal<f64>, index: usize, class: String) -> Element {
	let state = use_context::<State>(scope).unwrap();

	// FIXME: A wrapper is needed here... not sure exactly why (`leptos@0.0.22`).
	view!(scope,
		<svg>
			<path 
				class=format!("shape {}", class)
				d=move || state.scene.shapes[index].path(width, height(), &state.viewport().matrix)
			/>
		</svg>
	)
}

#[component]
pub fn Overlay (scope: Scope) -> Element {
	const WIDTH: f64 = 1_000.;


	let state = use_context::<State>(scope).unwrap();

	let height = {
		let state = state.clone();

		create_memo(scope, move |_| {
			let value = state.camera().aspect_ratio.split_once('/').unwrap();

			(WIDTH / value.0.parse::<f64>().unwrap() * value.1.parse::<f64>().unwrap()).ceil()
		})
	};

	view!(scope,
		<svg 
			class="canvas" 
			viewBox=move || format!("0 0 {:.0} {:.0}", WIDTH, height())
			xmlns="http://www.w3.org/2000/svg"
		>
			{move || state.camera().styles
				.iter()
				.map(|style| view!(scope,
					<Path 
						class=style.name.clone()
						height=height.into()
						index=style.index
						width=WIDTH
					/>
				))
				.collect::<Vec<_>>()}
		</svg>
	)
}
