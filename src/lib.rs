// (次の行を記述することで選択した Clippy ルールを無視します。
// - これは、`cargo make verify` でコードをチェックしたいが、
// いくつかのルールがあまりにも"煩わしい"、または書きたいコードに適用できない場合に便利です。)
#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

// ----- -----
//     Init
// ----- -----

// `init` にはアプリ起動時に初期化すべきことを記述します。
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model { counter : 0 }
}

// ----- -----
//    Model
// ----- -----

// `Model` にはアプリの状態を保持します。
struct Model {
    counter : i32,
}

// ----- -----
//   Update
// ----- -----

// (`Msg` 内に Copy trait を実装していない変数がある場合は、次の行を削除してください。)
#[derive(Copy, Clone)]
// `Msg` には、状態を変更する様々なイベントを記述します。
enum Msg {
    Increment,
}

// `update` にはそれぞれの `Msg` をハンドルする処理を記述します。
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => model.counter += 1,
    }
}

// ----- -----
//    View
// ----- -----

// `view` には表示するものを記述します。
fn view(model: &Model) -> Node<Msg> {
    div![
        "This is a counter: ",
        C!["counter"],
        button![model.counter, ev(Ev::Click, |_| Msg::Increment),],
    ]
}

// ----- -----
//    Start
// ----- -----

// (この関数は、`index.html` にある `init` 関数から呼び出されます。)
#[wasm_bindgen(start)]
pub fn start() {
    // `app` を`id = "app"` をもつ element にマウントします。
    App::start("app", init, update, view);
}
