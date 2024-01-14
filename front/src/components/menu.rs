use perseus::prelude::*;
use sycamore::prelude::*;

#[component]
pub fn MenuBlock<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
    nav(d-comp="navigation", class="defaultscroll is-sticky", id="topnav") {
      p(class="flex h-10 items-center justify-center bg-sky-500 px-4 text-sm font-medium text-white sm:px-6 lg:px-8") {
        "Tel: 0302"
      }
      div(class="container") {
        a(class="logo pl-0", href="index.html") {
          img(src="assets/images/logo-wide.png", class="inline-block dark:hidden", alt="") {
          }
          img(src="assets/images/logo-wide.png", class="hidden dark:inline-block", alt="") {
          }
        }
        div(class="menu-extras") {
          div(class="menu-item") {
            a(id="isToggle", class="navbar-toggle", onclick="toggleMenu()") {
              div(class="lines") {
                span() {
                }
                span() {
                }
                span() {
                }
              }
            }
          }
        }
        ul(class="buy-button list-none mb-0") {
          li(class="inline mb-0") {
            a(class="btn btn-icon rounded-full bg-blue-900/5 hover:bg-sky-600 border-blue-900/10 hover:border-sky-600 text-blue-900 hover:text-white", href="") {
              i(class="h-4 w-4", data-feather="settings") {
              }
            }
          }

        }
        div(id="navigation") {
          ul(class="navigation-menu") {
            li() {
              a(href="", class="sub-menu-item") {
                "Home"
              }
            }
            li(class="has-submenu parent-parent-menu-item") {
              a(href="latest") {
                "Latest"
              }
            }
            li(class="has-submenu parent-parent-menu-item") {
              a(href="") {
                "Events"
              }
            }
            li(class="has-submenu parent-parent-menu-item") {
              a(href="libraries") {
                "Library"
              }
            }
            li(class="has-submenu parent-menu-item") {
              a(href="") {
                "Publications"
              }
            }
            li(class="navigation-menu") {
              a(href="") {
                "Support Us"
              }
            }
          }
        }
      }
    }



        }
}
