(() => {
    const registry = {};

    const theme = {
        bg: "#070707",
        grid: "rgba(255,255,255,0.05)",
        text: "#ece9e1",
        muted: "#7d7a73",
        yellow: "#ffd400",
        yellowSoft: "rgba(255,212,0,0.35)",
        white: "#ffffff",
        gray: "#3a3a3a",
        green: "#9be564",
        orange: "#ff9f43",
    };

    function canvas(stage) {
        const el = document.createElement("canvas");
        stage.appendChild(el);
        const g = el.getContext("2d");
        const api = { el, g, w: 0, h: 0, dpr: 1, onResize: null };
        const fit = () => {
            const r = stage.getBoundingClientRect();
            const dpr = window.devicePixelRatio || 1;
            if (r.width === api.w && r.height === api.h && dpr === api.dpr) return;
            api.w = r.width;
            api.h = r.height;
            api.dpr = dpr;
            el.width = Math.round(r.width * dpr);
            el.height = Math.round(r.height * dpr);
            el.style.width = r.width + "px";
            el.style.height = r.height + "px";
            g.setTransform(dpr, 0, 0, dpr, 0, 0);
            if (api.onResize) api.onResize(api.w, api.h);
        };
        new ResizeObserver(fit).observe(stage);
        fit();
        api.clear = (color) => {
            g.save();
            g.setTransform(1, 0, 0, 1, 0, 0);
            if (color) {
                g.fillStyle = color;
                g.fillRect(0, 0, el.width, el.height);
            } else {
                g.clearRect(0, 0, el.width, el.height);
            }
            g.restore();
        };
        api.pointer = (handler) => {
            const pos = (e) => {
                const r = el.getBoundingClientRect();
                return { x: e.clientX - r.left, y: e.clientY - r.top };
            };
            let down = false;
            el.addEventListener("pointerdown", (e) => {
                down = true;
                el.setPointerCapture(e.pointerId);
                handler("down", pos(e), e);
            });
            el.addEventListener("pointermove", (e) => handler(down ? "drag" : "move", pos(e), e));
            el.addEventListener("pointerup", (e) => {
                down = false;
                handler("up", pos(e), e);
            });
        };
        return api;
    }

    function controls(figure) {
        let bar = figure.querySelector(".sim-controls");
        if (!bar) {
            bar = document.createElement("div");
            bar.className = "sim-controls";
            const cap = figure.querySelector("figcaption");
            figure.insertBefore(bar, cap);
        }
        const fmt = (v, step) => {
            const d = String(step).includes(".") ? String(step).split(".")[1].length : 0;
            return Number(v).toFixed(d);
        };
        return {
            slider(label, { min, max, step = 1, value, unit = "" }, onChange) {
                const wrap = document.createElement("label");
                wrap.className = "sim-control";
                const span = document.createElement("span");
                span.textContent = label;
                const input = document.createElement("input");
                input.type = "range";
                input.min = min;
                input.max = max;
                input.step = step;
                input.value = value;
                const out = document.createElement("output");
                out.textContent = fmt(value, step) + unit;
                input.addEventListener("input", () => {
                    out.textContent = fmt(input.value, step) + unit;
                    onChange(Number(input.value));
                });
                wrap.append(span, input, out);
                bar.appendChild(wrap);
                return {
                    get value() {
                        return Number(input.value);
                    },
                    set(v) {
                        input.value = v;
                        out.textContent = fmt(v, step) + unit;
                    },
                };
            },
            button(label, onClick, primary = false) {
                const b = document.createElement("button");
                b.type = "button";
                b.className = "sim-btn" + (primary ? " primary" : "");
                b.textContent = label;
                b.addEventListener("click", () => onClick(b));
                bar.appendChild(b);
                return b;
            },
            toggle(label, checked, onChange) {
                const wrap = document.createElement("label");
                wrap.className = "sim-toggle";
                const input = document.createElement("input");
                input.type = "checkbox";
                input.checked = checked;
                input.addEventListener("change", () => onChange(input.checked));
                wrap.append(input, document.createTextNode(label));
                bar.appendChild(wrap);
                return input;
            },
        };
    }

    function hud(stage) {
        const el = document.createElement("div");
        el.className = "sim-hud";
        stage.appendChild(el);
        return (html) => {
            el.innerHTML = html;
        };
    }

    function loop(figure, step) {
        let visible = false;
        let paused = false;
        let last = 0;
        let t = 0;
        let raf = 0;
        const frame = (now) => {
            raf = 0;
            if (!visible || paused) return;
            const dt = last ? Math.min((now - last) / 1000, 1 / 20) : 1 / 60;
            last = now;
            t += dt;
            step(dt, t);
            raf = requestAnimationFrame(frame);
        };
        const kick = () => {
            if (!raf && visible && !paused) {
                last = 0;
                raf = requestAnimationFrame(frame);
            }
        };
        new IntersectionObserver((entries) => {
            visible = entries[0].isIntersecting;
            kick();
        }).observe(figure);
        return {
            pause() {
                paused = true;
                figure.classList.add("paused");
            },
            play() {
                paused = false;
                figure.classList.remove("paused");
                kick();
            },
            toggle() {
                paused ? this.play() : this.pause();
                return !paused;
            },
            get paused() {
                return paused;
            },
            redraw() {
                step(0, t);
            },
        };
    }

    function mount(figure) {
        const name = figure.dataset.sim;
        const stage = figure.querySelector(".sim-stage");
        const factory = registry[name];
        const loading = stage.querySelector(".sim-loading");
        if (!factory) {
            if (loading) loading.textContent = `unknown simulation "${name}"`;
            return;
        }
        let config = {};
        try {
            config = JSON.parse(figure.dataset.config || "{}");
        } catch (_) { }
        if (loading) loading.remove();
        try {
            factory({
                figure,
                stage,
                config,
                theme,
                canvas: () => canvas(stage),
                controls: () => controls(figure),
                hud: () => hud(stage),
                loop: (step) => loop(figure, step),
            });
        } catch (err) {
            console.error(err);
            stage.innerHTML = `<div class="sim-loading">simulation crashed: ${String(err.message || err)}</div>`;
        }
    }

    window.Sims = {
        theme,
        register(name, factory) {
            registry[name] = factory;
            if (document.readyState !== "loading") {
                document.querySelectorAll(`figure.sim[data-sim="${name}"]`).forEach((f) => {
                    if (!f.dataset.mounted) {
                        f.dataset.mounted = "1";
                        mount(f);
                    }
                });
            }
        },
    };

    function renderMath() {
        if (!window.katex) return;
        document.querySelectorAll(".math").forEach((el) => {
            const display = el.classList.contains("math-display");
            try {
                window.katex.render(el.textContent, el, {
                    displayMode: display,
                    throwOnError: false,
                    strict: "ignore",
                });
            } catch (err) {
                console.warn(err);
            }
        });
    }

    function graphTooltips() {
        const tip = document.getElementById("tooltip");
        if (!tip) return;
        const show = (cell) => {
            const count = Number(cell.dataset.count || 0);
            const label = cell.dataset.label;
            if (!label) return;
            const titles = (cell.dataset.titles || "").split("|").filter(Boolean);
            const head = count
                ? `<b>${count} problem${count === 1 ? "" : "s"}</b> on ${label}`
                : `No posts on ${label}`;
            tip.innerHTML = head;
            if (titles.length) {
                const ul = document.createElement("ul");
                titles.forEach((t) => {
                    const li = document.createElement("li");
                    li.textContent = t;
                    ul.appendChild(li);
                });
                tip.appendChild(ul);
            }
            const r = cell.getBoundingClientRect();
            tip.hidden = false;
            const half = tip.offsetWidth / 2;
            const x = Math.min(Math.max(r.left + r.width / 2, half + 8), window.innerWidth - half - 8);
            tip.style.left = x + "px";
            tip.style.top = r.top + "px";
        };
        const hide = () => {
            tip.hidden = true;
        };
        document.querySelectorAll(".graph-grid .cg-cell[data-label]").forEach((cell) => {
            cell.addEventListener("mouseenter", () => show(cell));
            cell.addEventListener("mouseleave", hide);
            cell.addEventListener("focus", () => show(cell));
            cell.addEventListener("blur", hide);
        });
        window.addEventListener("scroll", hide, { passive: true });
        const scroller = document.querySelector(".graph-scroll");
        if (scroller) scroller.scrollLeft = scroller.scrollWidth;
    }

    document.addEventListener("DOMContentLoaded", () => {
        renderMath();
        if (window.hljs) window.hljs.highlightAll();
        graphTooltips();
        document.querySelectorAll("figure.sim").forEach((f) => {
            if (!f.dataset.mounted) {
                f.dataset.mounted = "1";
                mount(f);
            }
        });
    });
})();
