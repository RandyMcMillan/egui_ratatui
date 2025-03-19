default:
    @just --choose

bevy_example:
    @cargo run --bin bevy_example
    
gnostr-chat:
    @cargo b
    @cargo run --bin gnostr-chat
    
gnostr_chat_example:
    @cargo run --bin gnostr_chat_example
