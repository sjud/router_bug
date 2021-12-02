Type cargo run in server directory and go to 127.0.0.1:8000, you'll see NotFound
appear in the console as opposed to Index, despite index having the #[at("/")] attribute.

This code didn't have any yew-router bugs in 0.18 but now won't parse the browsers url,
for example it matches our route at #[not_found] where it should match our route at #[at("/")]