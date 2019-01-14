! version = 2.0

// Learn stuff about our users.

+ my name is *
- <set name=<formal>>Nice to meet you, <get name>.
- <set name=<formal>><get name>, nice to meet you.

+ my name is <bot master>
- <set name=<bot master>>That's my master's name too.

+ my name is <bot name>
- <set name=<bot name>>What a coincidence! That's my name too!
- <set name=<bot name>>That's my name too!

+ call me *
- <set name=<formal>><get name>, I will call you that from now on.

+ i am * years old
- <set age=<star>>A lot of people are <get age>, you're not alone.
- <set age=<star>>Cool, I'm <bot age> myself.{weight=49}

+ i am a (@malenoun)
- <set sex=male>Alright, you're a <star>.

+ i am a (@femalenoun)
- <set sex=female>Alright, you're female.

+ i (am from|live in) *
- <set location={formal}<star2>{/formal}>I've spoken to people from <get location> before.

+ my favorite * is *
- <set fav<star1>=<star2>>Why is it your favorite?

+ i am single
- <set status=single><set spouse=nobody>I am too.

+ i have a girlfriend
- <set status=girlfriend>What's her name?

+ i have a boyfriend
- <set status=boyfriend>What's his name?

+ *
% whats her name
- <set spouse=<formal>>That's a pretty name.

+ *
% whats his name
- <set spouse=<formal>>That's a cool name.

+ my (girlfriend|boyfriend)* name is *
- <set spouse=<formal>>That's a nice name.

+ (what is my name|who am i|do you know my name|do you know who i am){weight=10}
* <get name> == undefined => You never told me your name.
- Your name is <get name>.
- You told me your name is <get name>.
- Aren't you <get name>?

+ (how old am i|do you know how old i am|do you know my age){weight=10}
* <get age> == undefined => I don't know.
- You are <get age> years old.
- You're <get age>.

+ am i a (@malenoun) or a (@femalenoun){weight=10}
* <get sex> == undefined => I don't know.
- You're a <get sex>.

+ am i (@malenoun) or (@femalenoun){weight=10}
* <get sex> == undefined => I don't know.
- You're a <get sex>.

+ what is my favorite *{weight=10}
* <get fav<star>> == undefined => I don't know.
- Your favorite <star> is <get fav<star>>

+ who is my (boyfriend|girlfriend|spouse){weight=10}
* <get spouse> == undefined => I don't know.
- <get spouse>.
