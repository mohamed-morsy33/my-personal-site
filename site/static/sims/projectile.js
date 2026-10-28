Sims.register("projectile", ({ config, theme, canvas, controls, hud, loop }) => {
    const p = {
        speed: config.speed ?? 30,
        angle: config.angle ?? 45,
        drag: config.drag ?? 0.02,
        g: config.g ?? 9.81,
        playback: config.playback ?? 1.5,
    };

    let path = [];
    let ideal = null;
    let t = 0;

    const solve = () => {
        const th = (p.angle * Math.PI) / 180;
        const vx0 = p.speed * Math.cos(th);
        const vy0 = p.speed * Math.sin(th);
        const T = (2 * vy0) / p.g;
        ideal = { vx0, vy0, T, R: vx0 * T, H: (vy0 * vy0) / (2 * p.g) };

        const f = ([, , vx, vy]) => {
            const v = Math.hypot(vx, vy);
            return [vx, vy, -p.drag * v * vx, -p.g - p.drag * v * vy];
        };
        const h = 0.004;
        let y = [0, 0, vx0, vy0];
        let time = 0;
        path = [{ t: 0, x: 0, y: 0 }];
        while (time < 600) {
            const add = (a, b, k) => a.map((v, i) => v + b[i] * k);
            const k1 = f(y);
            const k2 = f(add(y, k1, h / 2));
            const k3 = f(add(y, k2, h / 2));
            const k4 = f(add(y, k3, h));
            const next = y.map((v, i) => v + (h / 6) * (k1[i] + 2 * k2[i] + 2 * k3[i] + k4[i]));
            time += h;
            if (next[1] < 0) {
                const a = y[1] / (y[1] - next[1]);
                path.push({ t: time - h + a * h, x: y[0] + a * (next[0] - y[0]), y: 0 });
                break;
            }
            y = next;
            path.push({ t: time, x: y[0], y: y[1] });
        }
        t = 0;
    };

    const cv = canvas();
    const { g } = cv;
    const setHud = hud();

    const frame = () => {
        const pad = { l: 46, r: 24, t: 70, b: 36 };
        const maxX = Math.max(ideal.R, 1) * 1.05;
        const maxY = Math.max(ideal.H, 1) * 1.15;
        const sx = (cv.w - pad.l - pad.r) / maxX;
        const sy = (cv.h - pad.t - pad.b) / maxY;
        const s = Math.min(sx, sy);
        return {
            X: (x) => pad.l + x * s,
            Y: (y) => cv.h - pad.b - y * s,
            s,
            pad,
            maxX: (cv.w - pad.l - pad.r) / s,
        };
    };

    const niceStep = (span) => {
        const raw = span / 6;
        const mag = Math.pow(10, Math.floor(Math.log10(raw)));
        const n = raw / mag;
        return (n < 1.5 ? 1 : n < 3.5 ? 2 : n < 7.5 ? 5 : 10) * mag;
    };

    const at = (time) => {
        if (time >= path[path.length - 1].t) return path[path.length - 1];
        let lo = 0;
        let hi = path.length - 1;
        while (hi - lo > 1) {
            const mid = (lo + hi) >> 1;
            path[mid].t <= time ? (lo = mid) : (hi = mid);
        }
        return path[lo];
    };

    const draw = () => {
        cv.clear();
        const { X, Y, maxX } = frame();

        g.strokeStyle = theme.gray;
        g.lineWidth = 1;
        g.beginPath();
        g.moveTo(X(0), Y(0));
        g.lineTo(X(maxX), Y(0));
        g.stroke();
        g.fillStyle = theme.muted;
        g.font = "12px Sniglet, sans-serif";
        g.textAlign = "center";
        const step = niceStep(maxX);
        for (let x = 0; x <= maxX; x += step) {
            g.fillRect(X(x), Y(0), 1, 5);
            g.fillText(`${Math.round(x)}m`, X(x), Y(0) + 18);
        }

        g.setLineDash([5, 6]);
        g.strokeStyle = "rgba(200,200,200,0.45)";
        g.lineWidth = 1.5;
        g.beginPath();
        const N = 120;
        for (let i = 0; i <= N; i++) {
            const tt = (ideal.T * i) / N;
            const x = ideal.vx0 * tt;
            const y = ideal.vy0 * tt - 0.5 * p.g * tt * tt;
            i ? g.lineTo(X(x), Y(y)) : g.moveTo(X(x), Y(y));
        }
        g.stroke();
        g.setLineDash([]);

        const now = at(t);
        g.strokeStyle = theme.yellow;
        g.lineWidth = 2.5;
        g.shadowColor = theme.yellowSoft;
        g.shadowBlur = 10;
        g.beginPath();
        for (let i = 0; i < path.length && path[i].t <= t; i += 2) {
            i ? g.lineTo(X(path[i].x), Y(path[i].y)) : g.moveTo(X(path[i].x), Y(path[i].y));
        }
        g.lineTo(X(now.x), Y(now.y));
        g.stroke();
        g.shadowBlur = 0;

        if (t <= ideal.T) {
            const ix = ideal.vx0 * t;
            const iy = ideal.vy0 * t - 0.5 * p.g * t * t;
            g.fillStyle = "#888";
            g.beginPath();
            g.arc(X(ix), Y(iy), 5, 0, Math.PI * 2);
            g.fill();
        }

        g.fillStyle = theme.yellow;
        g.beginPath();
        g.arc(X(now.x), Y(now.y), 7, 0, Math.PI * 2);
        g.fill();

        const land = path[path.length - 1];
        const peak = path.reduce((m, q) => (q.y > m ? q.y : m), 0);
        setHud(
            `<span style="color:#aaa">- - vacuum</span> &nbsp; <b>—</b> drag k=${p.drag}<br>` +
            `range <b>${land.x.toFixed(1)}</b> m vs ${ideal.R.toFixed(1)} m &nbsp; ` +
            `peak <b>${peak.toFixed(1)}</b> m vs ${ideal.H.toFixed(1)} m &nbsp; ` +
            `flight <b>${land.t.toFixed(2)}</b> s vs ${ideal.T.toFixed(2)} s`
        );
    };

    const sim = loop((dt) => {
        const end = Math.max(path[path.length - 1].t, ideal.T);
        if (t < end) t = Math.min(end, t + dt * p.playback);
        draw();
    });

    cv.onResize = draw;
    solve();
    draw();

    const ui = controls();
    ui.button("launch", () => {
        t = 0;
        sim.play();
    }, true);
    const relaunch = () => {
        solve();
        draw();
    };
    ui.slider("angle", { min: 5, max: 85, step: 1, value: p.angle, unit: "°" }, (v) => {
        p.angle = v;
        relaunch();
    });
    ui.slider("v₀", { min: 5, max: 80, step: 1, value: p.speed, unit: " m/s" }, (v) => {
        p.speed = v;
        relaunch();
    });
    ui.slider("k", { min: 0, max: 0.1, step: 0.002, value: p.drag }, (v) => {
        p.drag = v;
        relaunch();
    });
});
