use dioxus::prelude::*;

pub fn Login(cx: Scope) -> Element {
    render! {
        section { class: "bg-gray-50 dark:bg-gray-900",
            div { class: "flex flex-col items-center justify-center px-6 py-8 mx-auto md:h-screen lg:py-0",
                a {
                    href: "#",
                    class: "flex items-center mb-6 text-2xl font-semibold text-gray-900 dark:text-white",
                    img {
                        alt: "logo",
                        src: "https://flowbite.s3.amazonaws.com/blocks/marketing-ui/logo.svg",
                        class: "w-8 h-8 mr-2"
                    }
                    "Flowbite"
                }
                div { class: "w-full bg-white rounded-lg shadow dark:border md:mt-0 sm:max-w-md xl:p-0 dark:bg-gray-800 dark:border-gray-700",
                    div { class: "p-6 space-y-4 md:space-y-6 sm:p-8",
                        h1 { class: "text-xl font-bold leading-tight tracking-tight text-gray-900 md:text-2xl dark:text-white",
                            "Sign in to your account"
                        }
                        form { action: "#", class: "space-y-4 md:space-y-6",
                            div {
                                label {
                                    r#for: "email",
                                    class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                                    "Your email"
                                }
                                input {
                                    r#type: "email",
                                    name: "email",
                                    required: "",
                                    placeholder: "name@company.com",
                                    class: "bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                                    id: "email"
                                }
                            }
                            div {
                                label {
                                    r#for: "password",
                                    class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                                    "Password"
                                }
                                input {
                                    placeholder: "••••••••",
                                    required: "",
                                    name: "password",
                                    r#type: "password",
                                    class: "bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                                    id: "password"
                                }
                            }
                            div { class: "flex items-center justify-between",
                                div { class: "flex items-start",
                                    div { class: "flex items-center h-5",
                                        input {
                                            r#type: "checkbox",
                                            aria_describedby: "remember",
                                            required: "",
                                            class: "w-4 h-4 border border-gray-300 rounded bg-gray-50 focus:ring-3 focus:ring-primary-300 dark:bg-gray-700 dark:border-gray-600 dark:focus:ring-primary-600 dark:ring-offset-gray-800",
                                            id: "remember"
                                        }
                                    }
                                    div { class: "ml-3 text-sm",
                                        label {
                                            r#for: "remember",
                                            class: "text-gray-500 dark:text-gray-300",
                                            "Remember me"
                                        }
                                    }
                                }
                                a {
                                    href: "#",
                                    class: "text-sm font-medium text-primary-600 hover:underline dark:text-primary-500",
                                    "Forgot password?"
                                }
                            }
                            button {
                                r#type: "submit",
                                class: "w-full text-white bg-primary-600 hover:bg-primary-700 focus:ring-4 focus:outline-none focus:ring-primary-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-primary-600 dark:hover:bg-primary-700 dark:focus:ring-primary-800",
                                "Sign in"
                            }
                            p { class: "text-sm font-light text-gray-500 dark:text-gray-400",
                                "Don’t have an account yet? "
                                a {
                                    href: "#",
                                    class: "font-medium text-primary-600 hover:underline dark:text-primary-500",
                                    "Sign up"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
