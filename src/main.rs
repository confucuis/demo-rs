// use echo;
// use chat;
// use proxy;
use interface;

fn main() {
    // echo::start_server();
    // chat::start_server();
    // proxy::start_server();

    let cat = interface::Cat;
    let dog = interface::Dog;
    interface::sound_wawa(&cat);
    interface::sound_meme(&dog);
}
