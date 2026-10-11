## insights 

Service for gathering insights during event preparing created for
[clubvent](https://github.com/dupreehkuda/clubvent). 

`/restore` lets people paste an old event or summary link and copy or open its
updated address. Conversion happens in the browser and preserves the event ID;
it does not modify events or reopen closed ones. Share this page when an old
Telegram link no longer opens. It cannot redirect a request sent to a retired IP.

Check the link converter with `node --test tests/restore-link.test.mjs`.
