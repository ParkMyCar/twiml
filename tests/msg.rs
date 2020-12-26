use twiml::{Msg, Response, Twiml};

#[test]
fn test_multiple_messages() {
    let resp = Response::new()
        .msg(Msg::new("Welcome to Kino!"))
        .msg(Msg::new("Movie")
            .media("https://www.google.com/url?sa=i&url=https%3A%2F%2Fwww.dropbox.com%2F&psig=AOvVaw3JIxqD2FXHPHW8hhUkQlqj&ust=1609110922377000&source=images&cd=vfe&ved=0CAIQjRxqFwoTCOjL4rrj7O0CFQAAAAAdAAAAABAD")
        )
        .build()
        .unwrap();

    println!("{}", resp);
}
