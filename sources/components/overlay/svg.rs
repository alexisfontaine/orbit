use std::iter::once;

use leptos::*;

use crate::model::Style;
use crate::state::use_state;


#[component]
pub fn Overlay (scope: Scope) -> impl IntoView {
	const WIDTH: f64 = 1_000.;


	let state = use_state(scope);

	let height = Signal::derive(scope, move || state.with_camera(|camera| {
		let value = camera.aspect_ratio.split_once('/').unwrap();

		(WIDTH / value.0.parse::<f64>().unwrap() * value.1.parse::<f64>().unwrap()).ceil()
	}));

	view!(scope,
		<svg
			class="overlay"
			viewBox=move || format!("0 0 {WIDTH:.0} {:.0}", height())
			xmlns="http://www.w3.org/2000/svg"
		>
			<Paths
				height
				indexes=Signal::derive(scope, move || state.with_camera(|_| Vec::new()))
				width=WIDTH
			/>
		</svg>
	)
}

#[component]
fn Paths (scope: Scope, width: f64, height: Signal<f64>, indexes: Signal<Vec<usize>>) -> impl IntoView {
	let state = use_state(scope);

	let groups = move || state.with_camera(|camera| indexes.with(|indexes| {
		let mut styles = &camera.styles;

		for &index in indexes {
			styles = &styles.get(index)?.get_compound().unwrap().children;
		}

		Some(styles
			.iter()
			.filter_map(Style::get_compound)
			.count())
	}));

	let shapes = move || state.with_camera(|camera| indexes.with(|indexes| {
		let mut styles = &camera.styles;

		for &index in indexes {
			styles = &styles.get(index)?.get_compound().unwrap().children;
		}

		Some(styles
			.iter()
			.filter_map(Style::get_shape)
			.count())
	}));

	view!(scope,
		<For
			each=move || 0..groups().unwrap_or_default()
			key=|&index| index
			view=move |index| view!(scope,
				<g
					class=move || state.with_camera(|camera| indexes.with(|indexes| {
						let mut styles = &camera.styles;

						for &index in indexes {
							styles = &styles[index].get_compound().unwrap().children;
						}

						Some(styles.get(index)?.get_compound().unwrap().name.clone())
					}))
				>
					<Paths
						height
						indexes=Signal::derive(scope, move || indexes.with(|indexes| indexes.iter().copied().chain(once(index)).collect()))
						width
					/>
				</g>
			)
		/>

		<For
			each=move || 0..shapes().unwrap_or_default()
			key=|&index| index
			view=move |index| view!(scope,
				<path
					class=move || state.with_camera(|camera| indexes.with(|indexes| {
						let mut styles = &camera.styles;

						for &index in indexes {
							styles = &styles.get(index)?.get_compound().unwrap().children;
						}

						Some(format!("shape {}", &styles.get(index)?.get_shape().unwrap().name))
					}))
					d=move || state.with_camera(|camera| indexes.with(|indexes| {
						let mut styles = &camera.styles;

						for &index in indexes {
							styles = &styles.get(index)?.get_compound().unwrap().children;
						}

						state
							.get_scene().shapes[styles.get(index)?.get_shape().unwrap().index]
							.path(width, height(), &camera.viewports[state.get_viewport()].matrix)
					}))
				/>
			)
		/>
	)
}
