/**
 * Returns true if browser supports HTML5 localStorage.
 */
function supportsHTML5Storage() {
    try {
        return 'localStorage' in window && window['localStorage'] !== null;
    } catch (e) {
        return false;
    }
}

/**
 * Event handler for when a tab is clicked. 
 */
function onClickTab(event) {
    var target = event.currentTarget;
    var language = target.id;

    switchAllTabs(language);
}

/**
 * Switches the displayed tab for the given container.
 *
 * :param container: The div containing both the tab bar and the individual tabs
 * as direct children.
 * :param language: The language to switch to.
 */
function switchTab(container, language) {
    for (i = 1; i < container.children.length; ++i) {
        var tab = container.children[i];
        if (tab.id === language) {
            tab.style.display = "block";
        } else {
            tab.style.display = "none";
        }
    }

    var tab_container = container.children[0]
    for (i = 0; i < tab_container.children.length; ++i) {
        var button = tab_container.children[i];
        button.className = button.className.replace(" active", "");
        if (button.id === language) {
            button.className += " active";
        }
    }
}

/**
 * Switches all tabs on the page to the given language.
 *
 * :param language: The language to switch to.
 */
function switchAllTabs(language) {
    var containers = document.getElementsByClassName("tabs");
    for (i = 0; i < containers.length; ++i) {
        switchTab(containers[i], language);
    }

    if (supportsHTML5Storage()) {
        localStorage.setItem("glean-preferred-language", language);
    }
}

/**
 * Opens all tabs on the page to the given language on page load.
 */
function openTabs() {
    if (!supportsHTML5Storage()) {
        return;
    }

    var language = localStorage.getItem("glean-preferred-language");
    if (language == null) {
        language = "Kotlin";
    }

    switchAllTabs(language);
}

openTabs()
