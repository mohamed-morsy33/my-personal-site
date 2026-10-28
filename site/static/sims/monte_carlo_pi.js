Sims.register("monte_carlo_pi", ({ config, theme, canvas, controls, hud, loop }) => {
    const RES = 640;
    const layer = document.createElement("canvas");
    layer.width = RES;
    layer.height = RES;
    const lg = layer.getContext("2d");

    let rate = config.rate ?? 40;
    const maxDrawn = config.max_points ?? 80000;
    let total = 0;
    let inside = 0;
    let history = [];
    let stride = 1;
    let sinceSample = 0;

    const resetLayer = () => {
        lg.fillStyle = theme.bg;
        lg.fillRect(0, 0, RES, RES);
    };

    const reset = () => {
        total = 0;
        inside = 0;
        history = [];
        stride = 1;
        sinceSample = 0;
        resetLayer();
    };
    reset();

    const cv = canvas();
    const { g } = cv;
    const setHud = hud();

    const layout = () => {
        const pad = 16;
        const side = Math.min(cv.h - pad * 2, cv.w - pad * 2);
        const wide = cv.w - side - pad * 3 > 220;
        const sq = wide
            ? { x: pad, y: (cv.h - side) / 2, s: side }
            : { x: (cv.w - side) / 2, y: pad, s: side };
        const chart = wide ? { x: sq.x + side + pad * 2, y: pad + 60, w: cv.w - side - pad * 4, h: cv.h - pad * 2 - 80 } : null;
        return { sq, chart };
    };

    const addPoints = (n) => {
        for (let i = 0; i < n; i++) {
            const x = Math.random();
            const y = Math.random();
            const hit = x * x + y * y <= 1;
            total++;
            if (hit) inside++;
            if (total <= maxDrawn) {
                lg.fillStyle = hit ? "rgba(255,212,0,0.85)" : "rgba(140,140,140,0.55)";
                lg.fillRect(x * RES, (1 - y) * RES, 2, 2);
            }
            sinceSample++;
            if (sinceSample >= stride) {
                sinceSample = 0;
                history.push((4 * inside) / total);
                if (history.length > 600) {
                    history = history.filter((_, j) => j % 2 === 0);
                    stride *= 2;
                }
            }
        }
    };

    const draw = () => {
        cv.clear();
        const { sq, chart } = layout();
        g.drawImage(layer, sq.x, sq.y, sq.s, sq.s);
        g.strokeStyle = theme.yellow;
        g.lineWidth = 1.5;
        g.beginPath();
        g.arc(sq.x, sq.y + sq.s, sq.s, -Math.PI / 2, 0);
        g.stroke();
        g.strokeStyle = theme.gray;
        g.strokeRect(sq.x + 0.5, sq.y + 0.5, sq.s - 1, sq.s - 1);

        const est = total ? (4 * inside) / total : 0;
        if (chart && history.length > 1) {
            const lo = Math.PI - 0.35;
            const hi = Math.PI + 0.35;
            const Y = (v) => chart.y + chart.h * (1 - (Math.min(hi, Math.max(lo, v)) - lo) / (hi - lo));
            g.strokeStyle = theme.gray;
            g.lineWidth = 1;
            g.strokeRect(chart.x + 0.5, chart.y + 0.5, chart.w, chart.h);
            g.setLineDash([4, 5]);
            g.strokeStyle = "rgba(255,255,255,0.4)";
            g.beginPath();
            g.moveTo(chart.x, Y(Math.PI));
            g.lineTo(chart.x + chart.w, Y(Math.PI));
            g.stroke();
            g.setLineDash([]);
            g.fillStyle = theme.muted;
            g.font = "12px Sniglet, sans-serif";
            g.fillText("π", chart.x + chart.w - 14, Y(Math.PI) - 6);
            g.fillText("estimate vs samples", chart.x, chart.y - 10);
            g.strokeStyle = theme.yellow;
            g.lineWidth = 1.8;
            g.beginPath();
            history.forEach((v, i) => {
                const x = chart.x + (i / (history.length - 1)) * chart.w;
                i ? g.lineTo(x, Y(v)) : g.moveTo(x, Y(v));
            });
            g.stroke();
        }

        const err = total ? Math.abs(est - Math.PI) : 0;
        setHud(
            `N = <b>${total.toLocaleString()}</b><br>` +
            `inside = <b>${inside.toLocaleString()}</b><br>` +
            `π ≈ <b>${est.toFixed(6)}</b><br>` +
            `|error| = <b>${err.toFixed(6)}</b>`
        );
    };

    const sim = loop((dt) => {
        if (dt > 0) addPoints(rate);
        draw();
    });

    cv.onResize = draw;

    const ui = controls();
    ui.button("pause", (b) => {
        b.textContent = sim.toggle() ? "pause" : "play";
    }, true);
    ui.button("reset", () => {
        reset();
        draw();
    });
    ui.button("+10k", () => {
        addPoints(10000);
        draw();
    });
    ui.slider("points/frame", { min: 1, max: 1000, step: 1, value: rate }, (v) => (rate = v));
});
