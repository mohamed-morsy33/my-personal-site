Sims.register("fourier", ({ config, theme, canvas, controls, hud, loop }) => {
    const waves = {
        square: {
            label: "square",
            coeff: (k) => (k % 2 ? 4 / (Math.PI * k) : 0),
            f: (x) => (Math.sin(x) >= 0 ? 1 : -1),
        },
        sawtooth: {
            label: "sawtooth",
            coeff: (k) => ((k % 2 ? 2 : -2) / (Math.PI * k)),
            f: (x) => {
                const u = (((x + Math.PI) % (2 * Math.PI)) + 2 * Math.PI) % (2 * Math.PI);
                return (u - Math.PI) / Math.PI;
            },
        },
        triangle: {
            label: "triangle",
            coeff: (k) => (k % 2 ? ((((k - 1) / 2) % 2 ? -1 : 1) * 8) / (Math.PI * Math.PI * k * k) : 0),
            f: (x) => (2 / Math.PI) * Math.asin(Math.sin(x)),
        },
    };
    const order = Object.keys(waves);
    let wave = waves[config.wave] ? config.wave : "square";
    let terms = config.terms ?? 5;
    let speed = config.speed ?? 1;

    let harmonics = [];
    const build = () => {
        harmonics = [];
        for (let k = 1; harmonics.length < terms && k < 400; k++) {
            const b = waves[wave].coeff(k);
            if (b !== 0) harmonics.push({ k, b });
        }
    };
    build();

    const partial = (x) => harmonics.reduce((acc, { k, b }) => acc + b * Math.sin(k * x), 0);

    const cv = canvas();
    const { g } = cv;
    const setHud = hud();

    const draw = (t) => {
        cv.clear();
        const R = Math.min(cv.h * 0.3, cv.w * 0.13);
        const cx = R * 1.5 + 16;
        const cy = cv.h / 2 + 10;
        const x0 = cx + R * 1.7 + 24;
        const span = cv.w - x0 - 16;
        const pxPerRad = span / (4 * Math.PI);

        g.strokeStyle = theme.gray;
        g.lineWidth = 1;
        g.beginPath();
        g.moveTo(x0, cy);
        g.lineTo(cv.w - 16, cy);
        g.stroke();

        let x = cx;
        let y = cy;
        harmonics.forEach(({ k, b }, i) => {
            const r = Math.abs(b) * R;
            const a = k * t + (b < 0 ? Math.PI : 0);
            g.strokeStyle = `rgba(255,255,255,${Math.max(0.06, 0.28 - i * 0.02)})`;
            g.beginPath();
            g.arc(x, y, r, 0, Math.PI * 2);
            g.stroke();
            const nx = x + r * Math.cos(a);
            const ny = y - r * Math.sin(a);
            g.strokeStyle = "rgba(255,212,0,0.7)";
            g.beginPath();
            g.moveTo(x, y);
            g.lineTo(nx, ny);
            g.stroke();
            x = nx;
            y = ny;
        });

        g.fillStyle = theme.yellow;
        g.beginPath();
        g.arc(x, y, 4, 0, Math.PI * 2);
        g.fill();

        g.setLineDash([3, 4]);
        g.strokeStyle = "rgba(255,212,0,0.45)";
        g.beginPath();
        g.moveTo(x, y);
        g.lineTo(x0, y);
        g.stroke();
        g.setLineDash([]);

        g.strokeStyle = "rgba(200,200,200,0.35)";
        g.lineWidth = 1.5;
        g.beginPath();
        for (let px = 0; px <= span; px += 1) {
            const v = waves[wave].f(t - px / pxPerRad);
            const yy = cy - v * R;
            px ? g.lineTo(x0 + px, yy) : g.moveTo(x0 + px, yy);
        }
        g.stroke();

        g.strokeStyle = theme.yellow;
        g.lineWidth = 2.2;
        g.shadowColor = theme.yellowSoft;
        g.shadowBlur = 8;
        g.beginPath();
        for (let px = 0; px <= span; px += 1) {
            const yy = cy - partial(t - px / pxPerRad) * R;
            px ? g.lineTo(x0 + px, yy) : g.moveTo(x0 + px, yy);
        }
        g.stroke();
        g.shadowBlur = 0;

        const last = harmonics[harmonics.length - 1];
        setHud(`${waves[wave].label} wave · <b>${harmonics.length}</b> term${harmonics.length === 1 ? "" : "s"} · highest harmonic k = <b>${last ? last.k : 0}</b>`);
    };

    let phase = 0;
    loop((dt) => {
        phase += dt * speed;
        draw(phase);
    });

    cv.onResize = () => draw(phase);

    const ui = controls();
    const termsSlider = ui.slider("terms", { min: 1, max: 60, step: 1, value: terms }, (v) => {
        terms = v;
        build();
        draw(phase);
    });
    ui.button(`wave: ${wave}`, (b) => {
        wave = order[(order.indexOf(wave) + 1) % order.length];
        b.textContent = `wave: ${wave}`;
        build();
        draw(phase);
    });
    ui.slider("speed", { min: 0, max: 3, step: 0.1, value: speed }, (v) => (speed = v));
    termsSlider.set(terms);
});
