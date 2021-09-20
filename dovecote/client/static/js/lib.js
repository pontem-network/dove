HTMLElement.prototype.addClass = function (name) {
    var el = this;
    name.split(' ').map(function (name) {
        return name.trim();
    })
        .filter(function (name) {
            return name;
        })
        .forEach(function (name) {
            el.classList.add(name);
        });
    return this;
};
HTMLElement.prototype.removeClass = function (name) {
    var el = this;
    name.split(' ').map(function (name) {
        return name.trim();
    })
        .filter(function (name) {
            return name;
        })
        .forEach(function (name) {
            el.classList.remove(name);
        });
    return this;
};
HTMLElement.prototype.hasClass = function (name) {
    return this.classList.contains(name);
};
HTMLElement.prototype.toggleClass = function (name) {
    return this[this.hasClass(name) ? 'removeClass' : 'addClass'](name);
};
HTMLElement.prototype.attr = function (attr_name, attr_value) {
    if (attr_value === undefined) {
        return this.getAttribute(attr_name);
    }
    this.setAttribute(attr_name, attr_value);
    return this;
}
HTMLElement.prototype.event_run_name = function (name) {
    this.dispatchEvent((new Event(name, {bubbles: true, cancelable: true})));
    return this;
}
HTMLElement.prototype.change = function () {
    this.event_run_name('change');
    return this.event_run_name("onchange");
}
HTMLElement.prototype.click = function () {
    this.event_run_name("onclick");
    return this.event_run_name("click");
}
HTMLElement.prototype.submit = function () {
    this.event_run_name("onsubmit");
    return this.event_run_name("submit");
}
