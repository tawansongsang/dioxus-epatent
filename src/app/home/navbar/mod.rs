use dioxus::prelude::*;

pub fn NavBar(cx: Scope) -> Element {
    render! {
        nav {
            class: "bg-white border-gray-200 dark:bg-gray-900",
            div {
                class: "mx-auto max-w-7xl px-2 sm:px-6 lg:px-8",
                div {
                    class: "relative flex h-16 items-center justify-between",
                    LeftNavBar {}
                    RightNavBar {}
                    CenterNavBar {}
                }
            }
        }
    }
}

fn RightNavBar(cx: Scope) -> Element {
    render! {
        div {
            class: "flex items-center md:order-2 space-x-3 md:space-x-0",
            "right"
        }
    }
}

fn LeftNavBar(cx: Scope) -> Element {
    render! {
            a {
                class: "relative inline-flex items-center justify-center rounded-md p-2",
                href: "https://www.ipthailand.go.th",
                span {
                    class: "self-center text-2xl font-semibold whitespace-nowrap dark:text-white",
                    "DIP"
                }
            }
    }
}

fn CenterNavBar(cx: Scope) -> Element {
    render! {
        div {
            class: "items-center justify-between hidden w-full md:flex md:w-auto md:order-1",
            id: "navbar-user",
            ul { 
                class: "flex flex-col font-medium p-4 md:p-0 mt-4 border border-gray-100 rounded-lg bg-gray-50 md:space-x-8 md:flex-row md:mt-0 md:border-0 md:bg-white dark:bg-gray-800 md:dark:bg-gray-900 dark:border-gray-700",
                li {
                    a {
                        href: "#",
                        aria_current: "page",
                        class: "block py-2 px-3 text-white bg-blue-700 rounded md:bg-transparent md:text-blue-700 md:p-0 md:dark:text-blue-500",
                        "Home"
                    }
                }
                li {
                    a {
                        href: "#",
                        class: "block py-2 px-3 text-gray-900 rounded hover:bg-gray-100 md:hover:bg-transparent md:hover:text-blue-700 md:p-0 dark:text-white md:dark:hover:text-blue-500 dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent dark:border-gray-700",
                        "About"
                    }
                }
                li {
                    a {
                        href: "#",
                        class: "block py-2 px-3 text-gray-900 rounded hover:bg-gray-100 md:hover:bg-transparent md:hover:text-blue-700 md:p-0 dark:text-white md:dark:hover:text-blue-500 dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent dark:border-gray-700",
                        "Services"
                    }
                }
                li {
                    a {
                        href: "#",
                        class: "block py-2 px-3 text-gray-900 rounded hover:bg-gray-100 md:hover:bg-transparent md:hover:text-blue-700 md:p-0 dark:text-white md:dark:hover:text-blue-500 dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent dark:border-gray-700",
                        "Pricing"
                    }
                }
                li {
                    a {
                        href: "#",
                        class: "block py-2 px-3 text-gray-900 rounded hover:bg-gray-100 md:hover:bg-transparent md:hover:text-blue-700 md:p-0 dark:text-white md:dark:hover:text-blue-500 dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent dark:border-gray-700",
                        "Contact"
                    }
                }
            }
        }
    }
}
