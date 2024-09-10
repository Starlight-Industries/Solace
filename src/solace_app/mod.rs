// use freya::prelude::*;

// pub fn _app() -> Element {
//     let mut times = use_signal(|| 1);

//     let exclamations = "!".repeat(times());

//     rsx!(
//         rect {
//             width: "100%",
//             height: "100%",
//             background: "rgb(57, 138, 215)",
//             main_align: "center",
//             cross_align: "center",
//             onclick: move |_| times += 5,
//             label {
//                 width: "100%",
//                 font_size: "50",
//                 text_align: "center",
//                 color: "white",
//                 "Hello, World{exclamations}"
//             }
//         }
//     )
// }