Sims.register("double_pendulum", ({ config, theme, canvas, controls, hud, loop }) => {
    const deg = Math.PI / 180;
    const p = {
        l1: config.l1 ?? 1,
        l2: config.l2 ?? 1,
        m1: config.m1 ?? 1,
        m2: config.m2 ?? 1,
        g: config.g ?? 9.81,
        damping: config.damping ?? 0,
    };
    const init = [(config.theta1 ?? 120) * deg, 0, (config.theta2 ?? -20) * deg, 0];
    const ghostOffset = config.ghost ? (config.ghost_offset ?? 0.001) : null;
    const trailMax = config.trail_length ?? 500;

    let s, ghost, trail, ghostTrail, t;
    const reset = () => {
        s = init.slice();
        ghost = ghostOffset === null ? null : [init[0] + ghostOffset, 0, init[2], 0];
        trail = [];
        ghostTrail = [];
        t = 0;
    };
    reset();

    const deriv = ([t1, w1, t2, w2]) => {
        const { l1, l2, m1, m2, g, damping } = p;
        const d = t1 - t2;
        const den = 2 * m1 + m2 - m2 * Math.cos(2 * d);
        const a1 =
            (-g * (2 * m1 + m2) * Math.sin(t1) -
                m2 * g * Math.sin(t1 - 2 * t2) -
                2 * Math.sin(d) * m2 * (w2 * w2 * l2 + w1 * w1 * l1 * Math.cos(d))) /
            (l1 * den);
        const a2 =
            (2 *
                Math.sin(d) *
                (w1 * w1 * l1 * (m1 + m2) + g * (m1 + m2) * Math.cos(t1) + w2 * w2 * l2 * m2 * Math.cos(d))) /
            (l2 * den);
        return [w1, a1 - damping * w1, w2, a2 - damping * w2];
    };

    const rk4 = (y, h) => {
        const add = (a, b, k) => a.map((v, i) => v + b[i] * k);
        const k1 = deriv(y);
        const k2 = deriv(add(y, k1, h / 2));
        const k3 = deriv(add(y, k2, h / 2));
        const k4 = deriv(add(y, k3, h));
        return y.map((v, i) => v + (h / 6) * (k1[i] + 2 * k2[i] + 2 * k3[i] + k4[i]));
    };

    const energy = ([t1, w1, t2, w2]) => {
        const { l1, l2, m1, m2, g } = p;
        const y1 = -l1 * Math.cos(t1);
        const y2 = y1 - l2 * Math.cos(t2);
        const T =
            0.5 * m1 * l1 * l1 * w1 * w1 +
            0.5 * m2 * (l1 * l1 * w1 * w1 + l2 * l2 * w2 * w2 + 2 * l1 * l2 * w1 * w2 * Math.cos(t1 - t2));
        return T + m1 * g * y1 + m2 * g * y2;
    };

    const cv = canvas();
    const { g } = cv;
    const setHud = hud();
    let dragging = null;

    const geom = () => {
        const scale = (Math.min(cv.w, cv.h) * 0.46) / (p.l1 + p.l2);
        return { ox: cv.w / 2, oy: cv.h / 2, scale };
    };

    const positions = (y) => {
        const { ox, oy, scale } = geom();
        const x1 = ox + p.l1 * scale * Math.sin(y[0]);
        const y1 = oy + p.l1 * scale * Math.cos(y[0]);
        const x2 = x1 + p.l2 * scale * Math.sin(y[2]);
        const y2 = y1 + p.l2 * scale * Math.cos(y[2]);
        return { x1, y1, x2, y2 };
    };

    const drawTrail = (pts, color) => {
        for (let i = 1; i < pts.length; i++) {
            g.strokeStyle = color.replace("A", (i / pts.length).toFixed(3));
            g.lineWidth = 1.5;
            g.beginPath();
            g.moveTo(pts[i - 1][0], pts[i - 1][1]);
            g.lineTo(pts[i][0], pts[i][1]);
            g.stroke();
        }
    };

    const drawArm = (y, arm, bob, r) => {
        const { ox, oy } = geom();
        const { x1, y1, x2, y2 } = positions(y);
        g.strokeStyle = arm;
        g.lineWidth = 2;
        g.beginPath();
        g.moveTo(ox, oy);
        g.lineTo(x1, y1);
        g.lineTo(x2, y2);
        g.stroke();
        g.fillStyle = bob;
        g.beginPath();
        g.arc(x1, y1, r * Math.cbrt(p.m1), 0, Math.PI * 2);
        g.fill();
        g.beginPath();
        g.arc(x2, y2, r * Math.cbrt(p.m2), 0, Math.PI * 2);
        g.fill();
    };

    const draw = () => {
        cv.clear();
        const { ox, oy } = geom();
        if (ghost) drawTrail(ghostTrail, "rgba(150,150,150,A)");
        drawTrail(trail, "rgba(255,212,0,A)");
        if (ghost) drawArm(ghost, "rgba(160,160,160,0.5)", "#777", 7);
        g.shadowColor = theme.yellowSoft;
        g.shadowBlur = 16;
        drawArm(s, theme.text, theme.yellow, 9);
        g.shadowBlur = 0;
        g.fillStyle = theme.text;
        g.beginPath();
        g.arc(ox, oy, 4, 0, Math.PI * 2);
        g.fill();
        const e = energy(s);
        let html = `t = <b>${t.toFixed(2)}</b> s<br>E = <b>${e.toFixed(3)}</b> J<br>θ₁ = <b>${((s[0] / deg) % 360).toFixed(1)}°</b> θ₂ = <b>${((s[2] / deg) % 360).toFixed(1)}°</b>`;
        if (ghost) {
            const a = positions(s);
            const b = positions(ghost);
            const sep = Math.hypot(a.x2 - b.x2, a.y2 - b.y2) / geom().scale;
            html += `<br>separation = <b>${sep.toExponential(2)}</b> m`;
        }
        setHud(html);
    };

    const sim = loop((dt) => {
        if (!dragging && dt > 0) {
            const n = Math.max(1, Math.ceil(dt / (1 / 480)));
            const h = dt / n;
            for (let i = 0; i < n; i++) {
                s = rk4(s, h);
                if (ghost) ghost = rk4(ghost, h);
            }
            t += dt;
            const { x2, y2 } = positions(s);
            trail.push([x2, y2]);
            if (trail.length > trailMax) trail.shift();
            if (ghost) {
                const q = positions(ghost);
                ghostTrail.push([q.x2, q.y2]);
                if (ghostTrail.length > trailMax) ghostTrail.shift();
            }
        }
        draw();
    });

    cv.onResize = () => {
        trail = [];
        ghostTrail = [];
        draw();
    };

    cv.pointer((type, pt) => {
        const { ox, oy } = geom();
        if (type === "down") {
            const { x1, y1, x2, y2 } = positions(s);
            const d1 = Math.hypot(pt.x - x1, pt.y - y1);
            const d2 = Math.hypot(pt.x - x2, pt.y - y2);
            if (Math.min(d1, d2) < 40) dragging = d1 < d2 ? 1 : 2;
        } else if (type === "drag" && dragging) {
            if (dragging === 1) {
                s[0] = Math.atan2(pt.x - ox, pt.y - oy);
            } else {
                const { x1, y1 } = positions(s);
                s[2] = Math.atan2(pt.x - x1, pt.y - y1);
            }
            s[1] = 0;
            s[3] = 0;
            if (ghost) ghost = [s[0] + ghostOffset, 0, s[2], 0];
            trail = [];
            ghostTrail = [];
            t = 0;
            draw();
        } else if (type === "up") {
            dragging = null;
        }
    });

    const ui = controls();
    ui.button("pause", (b) => {
        b.textContent = sim.toggle() ? "pause" : "play";
    }, true);
    ui.button("reset", () => {
        reset();
        draw();
    });
    ui.slider("g", { min: 1, max: 25, step: 0.1, value: p.g, unit: " m/s²" }, (v) => (p.g = v));
    ui.slider("m₂/m₁", { min: 0.2, max: 5, step: 0.1, value: p.m2 / p.m1 }, (v) => (p.m2 = v * p.m1));
    ui.slider("damping", { min: 0, max: 0.5, step: 0.01, value: p.damping }, (v) => (p.damping = v));
});
