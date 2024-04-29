// use echo;
// use chat;
// use proxy;
use inferface;

fn main() {
    // echo::start_server();
    // chat::start_server();
    // proxy::start_server();

    let cat = inferface::Cat;
    let dog = inferface::Dog;
    inferface::sound_off(&cat);
    inferface::sound_off(&dog);
}
