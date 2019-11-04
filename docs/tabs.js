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
    let target = event.currentTarget;
    let language = target.dataset.lang;

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
    let tab_contents_container = container.children[1];
    for (i = 0; i < tab_contents_container.children.length; ++i) {
        let tab = tab_contents_container.children[i];
        if (tab.dataset.lang === language) {
            tab.style.visibility = "visible";
        } else {
            tab.style.visibility = "hidden";
        }
    }

    let tab_container = container.children[0];
    for (i = 0; i < tab_container.children.length; ++i) {
        let button = tab_container.children[i];
        button.className = button.className.replace(" active", "");
        if (button.dataset.lang === language) {
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
    let containers = document.getElementsByClassName("tabs");
    for (let i = 0; i < containers.length; ++i) {
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

    let containers = document.getElementsByClassName("tabs");
    for (let i = 0; i < containers.length; ++i) {
        // Create tabs for every language that has content
        let tabs = containers[i].children[0];
        let tabcontents = containers[i].children[1];
        for (let tabcontent of tabcontents.children) {
            let button = document.createElement("button");
            button.dataset.lang = tabcontent.dataset.lang;
            button.className = "tablinks";
            button.onclick = onClickTab;
            button.innerText = tabcontent.dataset.lang;
            tabs.appendChild(button);
        }

        // Set up the spacing and layout based on the number of active tabs
        let numTabs = tabcontents.children.length;
        tabcontents.style.width = `${numTabs * 100}%`;
        for (let j = 0; j < numTabs; ++j) {
            let tab = tabcontents.children[j];
            tab.style.transform = `translateX(-${j * 100}%)`;
            tab.style.width = `calc(${100 / numTabs}% - 26px)`;
        }
    }

    var language = localStorage.getItem("glean-preferred-language");
    if (language == null) {
        language = "Kotlin";
    }

    switchAllTabs(language);
}

openTabs()
