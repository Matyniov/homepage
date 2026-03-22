use dioxus::prelude::*;

use crate::router::Route;

const HEADING_CLASS: &str = "fancy_title text-3xl lg:text-5xl border-b-2 border-l-32 pl-2 mb-4";
const PARAGRAPH: &str = "indent-12 mb-5 text-justify";

#[component]
fn Hobby(image: Asset, title: String, accent: String, children: Element) -> Element {
    rsx! {
        div { class: "transition-all fancy_font w-60 h-80 skew-2 hover:skew-0 hover:scale-120 hover:z-10 flex flex-col items-center justify-start text-center bg-{accent} border-4 p-4",
            img { class: "mt-auto mb-auto max-h-40", src: image }
            p { class: "mt-auto font-bold border-b-4 w-full", {title} }
            p { class: "text-sm", {children} }
        }
    }
}

#[component]
pub fn WhoAmI() -> Element {
    rsx! {
        div {
            class: "flex flex-col items-center justify-center pt-20 gap-20 ani-entrance",
            div { class: "relative flex flex-col lg:flex-row gap-10 bg-white p-10 fancy_font black_dropshadow",
                div { class: "flex flex-col gap-4 bg-black items-center justify-between text-white p-10",
                    div {
                        p { class: "dorsa text-9xl leading-20", "MATYNIOV" }
                        p { class: "japanese translate origin-bottom-left scale-x-120",
                            "同性愛者、ゲイ"
                        }

                    }
                    img {
                        src: asset!("/assets/gif/digital man 2.gif"),
                        class: "size-50 grayscale",
                        class: "transition-all hover:scale-150",
                    }
                    p { class: "bar_code_font text-7xl", ".SOUL.FIRMWARE." }
                }
                div { class: "hidden lg:block relative border-40 size-150" }
                img {
                    src: asset!("/assets/imgs/bust.png"),
                    class: "hidden lg:block grayscale absolute right-0 top-1/2 top-1/2 -translate-y-1/2 size-170",
                }

            }

            div { class: "relative flex flex-col bg-white lg:transparent p-8 black_dropshadow lg:shadow-none",
                div { class: "hidden lg:block absolute top-1/2 left-1/2 -translate-y-1/2 -translate-x-1/2 skew-x-24 bg-white lg:w-140 lg:h-40 black_dropshadow" }
                div { class: "z-10",
                    p { class: "fancy_title  text-3xl text-left w-60 lg:w-100 lg:h-12 lg:-indent-8",
                        "\"Something something communism\""
                    }
                    p { class: "text-right content_serif italic text-2xl", "- Lenin" }

                }
            }

            div { class: "bg-black text-white p-8 lg:p-10 border-2 lg:border-40 black_dropshadow text-xl content_serif lg:max-w-250 mx-4 mb-20",
                p { class: HEADING_CLASS, "Hello world-wide-web!" }
                p { class: PARAGRAPH,
                    "I hope you are doing great in the cyberspace today. My name is Maty! I'm a surprisingly
                stereotypical computer science major from Poland (the queer kind, not the chud kind).
                Make of that what you will. Welcome to my website!"
                }

                div { class: "grid p-4 lg:grid-cols-3 gap-4 justify-around items-center",
                    Hobby {
                        image: asset!("/assets/gif/art.gif"),
                        title: "I make art sometimes!",
                        // bg-red-600
                        accent: "red-600",
                        Link { class: "underline", to: Route::Art {}, "Checkout out the art section!" }
                    }
                    Hobby {
                        image: asset!("/assets/gif/cooking.gif"),
                        title: "Cooking is real fun!",
                        // bg-teal-500
                        accent: "teal-500",
                        "Not an expert, but i know my way around the kitchen."
                    }
                    Hobby {
                        image: asset!("/assets/gif/cycling.gif"),
                        title: "Hey I'm cycling here!",
                        // bg-blue-600
                        accent: "blue-600",
                        "Recreationally and for commuitng, not competetively!"
                    }
                    Hobby {
                        image: asset!("/assets/gif/space.gif"),
                        title: "Spaaaaace!",
                        // bg-gray-800
                        accent: "gray-800",
                        "I like astronomy and space exploration. DON'T you dare mention astrology."
                    }
                    Hobby {
                        image: asset!("/assets/gif/photo.gif"),
                        title: "Caught you on film",
                        // bg-pink-800
                        accent: "pink-800",
                        "I take pictures sometimes, check out my flickr!"
                    }
                    Hobby {
                        image: asset!("/assets/gif/neo.gif"),
                        title: "Hacking the mainframe",
                        // bg-green-900
                        accent: "green-900",
                        "Comp-sci major, I code."
                    }
                    div {}
                    Hobby {
                        image: asset!("/assets/gif/travel.gif"),
                        title: "On my way",
                        // bg-orange-500
                        accent: "orange-500",
                        "Traveling is real fun but I haven't done much of that recently..."
                    }
                }
                p { class: "{PARAGRAPH} italic font-bold",
                    "Also, if you are a right winger or a fan of AI, you are unwelcome, fuck off."
                }

                p { class: HEADING_CLASS, "Meet my corporeal being!" }
                p { class: PARAGRAPH,
                    "If you happen to be in Warsaw or Tricity feel free to hit me up!
                     I loooooove meeting new people, especially IRL. Or just add me on barq,
                     cus I do in fact have an account on that app... "
                    Link { class: "underline", to: Route::Contact {},
                        "You can find contact info here!"
                    }
                }
                p { class: HEADING_CLASS, "What am I? What is my purpose?" }
                img {
                    class: "lg:float-right border-2 h-48 lg:ml-4 grayscale",
                    src: asset!("/assets/gif/deep in thought.gif"),
                }
                p { class: PARAGRAPH,
                    "I'm a huge fan of living - controversial, I know. I get to meet so many cool people, with so many stories.
                I get to do so many cool things, eat delicious food, see wonderful places, create art, experience the world while I last. 
                I am aware of the fact that all that I do will eventually 
                be consumed by decay. My whole existence and impact on the world reduced to 
                a thin film dispersed into obscurity across the earth by 
                the unrelenting forces of entropy.  I am at peace with this knowledge. 
                This does not stop me from pursuing fleeting moments of joy. I choose to
                fight my battles and not give in to the siren call of nihilism."
                }
                p { class: PARAGRAPH,
                    "I also believe the world can be better. And it can be better for everyone.
                    The road wont be easy but the arc of history will hopefully keep bending towards justice and all that is good.
                    So never give up, neither in your micro scale of personal life nor in the macro world wide scale."
                }

                p { class: HEADING_CLASS, "What the hell is this?" }
                p { class: PARAGRAPH,
                    "You might have noticed the style of this
                    website is all over the place. Some parts more pretentious then others. Thats on purpose! This is my first
                    time properly making a website, so I wanted to have some fun with the stylesheets! You can actually get the 
                    sourcecode for this website on my github!"
                }
            }
                // Interests glossary
        // Likes dislikes
        // Tell people to go look at your art or just contact you irl
        }

    }
}
