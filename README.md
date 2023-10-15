# How to add cards.

Run the code. It will be self explanatory.
When you run it in memory, you should start by creating a database tho.


# Steps to work with Twelve:

1. Mentally prepare yourself for torture.
2. Drink a beer.
3. Maybe a shot of Vodka would also be nice.
4. Procrastinating is also an option.
5. Atlas 12 is looking nice this time of the year.

*now for real*

## Getting the accounts and users:

1. Make a request to the /Accounts endpoint. There is a max of 100 at one request.
2. Iterate over the accounts, and save `id` and `lstUserIds` in a database. Becareful, because sometimes `lstUserIds` does not contain any UserIds.
3. Make a request to the /Users endpoint. There is a max of 10 000 at one request.
4. Iterate over the Users as well and save `id` and `email` (Hash the email for privacy) in a database. Once again, watch out because `email` can be a Null value.

## Adding a card to an user

1. Request their email address as input, and search for the Hash in the database
2. Search for the `id` linked to the `email` in the accounts table. NOTE: the `id` in the user table is called `UserId` in the account table and not `id`, there is an `id` but it is different.
3. When you find the account linked to the user, store the `id`'s of both tables in two variables for later use.
4. Now scan the NFC card. Make sure you get the whole serial number.
5. Also get the cvc / code on the top left corner of the card
6. Make a post request to the /Tokens endpoint. See `posttokens.rs` for more information on the right setup.

## Adding users
Enter the first name, the prefix and the lastname. Followed by their email address.
After this, you go to admin.twelve.eu and find the user on the user page. There you select the user and you click in the bottom left corner on `send login details`.


# API calls with Twelve

You survived the easy parts, congrats! Now lets talk about the horror that Twelve calls API calls.

There are 4 different types of API Calls:

- Get
- Post
- Delete
- Patch

That's pretty doable, just avoid using Delete and Patch as much as possible.

Each API Call can have up to 4 different types of filters on there:

- Query filter (Query filter is basically your path `?` followed by your filter and using & to use multiple)
- Path filter (Putting a filter directly in the Path before `?`, i.e. tokens/1 would fetch token 1)
- Body (Just use a JSON map)
- Header

Each call needs a few basic Headers:
```
  -H 'accept: text/plain' \
  -H 'PublicAPIKey: ....' \
  -H 'RequestToken: ....' \
  -H 'RequestSignature: ...' \
  -H 'ClientId: ....' \
  -H 'Content-Type: application/json' \
```
- Public api key is obvious, I hope.
- Request token is a token that starts with the current date followed by a random string of characters which needs to be unique every time!.
- Request signature is a SHA256 Hash existing out of the endpoint path formatted as `/api/v1/tokens`, the requesttoken, and your privatekey. Just stitch those 3 strings together and hash them. **It is very important to make these ASCII UPPERCASE characaters, otherwise it won't work.**
- The ClientId.

Well done! That was it.

In the end it was not that difficult after all. Especially with the documentation you just read;).

*A closing word about compiling for the tablets in the Internaat. These tablets don't have a proper vcruntime dll. To solve this, you have to build statically instead of dynamically. The code should default to this, but you can change this behaviour in the .cargo/config file.
Also building this release might sometimes also help: `cargo build --release --target x86_64-pc-windows-msvc`.