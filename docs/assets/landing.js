/**
 * Foruster site — GitHub releases API + idioma único (es | en).
 */
(function () {
  "use strict";

  var REPO = "m4rz3r0/foruster";
  var API = "https://api.github.com/repos/" + REPO + "/releases/latest";
  var STORAGE_KEY = "foruster-lang";

  function getLang() {
    return document.documentElement.classList.contains("lp-lang-en") ? "en" : "es";
  }

  function setLang(lang) {
    if (lang !== "es" && lang !== "en") return;
    document.documentElement.lang = lang;
    document.documentElement.className = "lp-root lp-lang-" + lang;
    try {
      localStorage.setItem(STORAGE_KEY, lang);
    } catch (e) {
      /* private mode */
    }
    document.title =
      lang === "en" ? "Foruster — downloads & guides" : "Foruster — descarga y guías";
    var sw = document.getElementById("lp-lang-switch");
    if (sw) sw.setAttribute("aria-label", lang === "es" ? "Idioma" : "Language");
    syncLangButtons();
    if (typeof window.forusterReloadDownloads === "function") {
      window.forusterReloadDownloads();
    }
  }

  function syncLangButtons() {
    var lang = getLang();
    document.querySelectorAll("[data-set-lang]").forEach(function (btn) {
      var sel = btn.getAttribute("data-set-lang") === lang;
      btn.setAttribute("aria-pressed", sel ? "true" : "false");
    });
  }

  function initLang() {
    document.querySelectorAll("[data-set-lang]").forEach(function (btn) {
      btn.addEventListener("click", function () {
        setLang(btn.getAttribute("data-set-lang") || "es");
      });
    });
    syncLangButtons();
  }

  function t(key) {
    var lang = getLang();
    var M = {
      loading: {
        es: "Cargando archivos del último release…",
        en: "Loading files from the latest release…",
      },
      empty: {
        es: "No hay archivos publicados en el último release.",
        en: "No files are attached to the latest release.",
      },
      fallback: {
        es: "No se pudieron cargar las descargas (red o límite de la API). Inténtelo de nuevo más tarde o solicite un enlace directo a quien le distribuya el software.",
        en: "Could not load downloads (network or API limit). Try again later or ask your distributor for a direct link.",
      },
      download: { es: "Descargar", en: "Download" },
    };
    return (M[key] && M[key][lang]) || "";
  }

  function classify(name) {
    var lower = name.toLowerCase();
    if (name === "SHA256SUMS") {
      return { os: "meta", role: "checksums", sort: 100 };
    }
    var isWin = lower.indexOf("windows") !== -1 || /\.exe$/i.test(name);
    var isLinux =
      lower.indexOf("linux") !== -1 ||
      /\.tar\.gz$/i.test(name) ||
      (lower.indexOf("linux") !== -1 && !/\.exe$/i.test(name));

    if (isWin && !isLinux) {
      if (lower.indexOf("bundle") !== -1) return { os: "win", role: "bundle", sort: 1 };
      if (lower.indexOf("installer") !== -1) return { os: "win", role: "installer", sort: 2 };
      return { os: "win", role: "app", sort: 3 };
    }
    if (isLinux || /\.tar\.gz$/i.test(name)) {
      if (lower.indexOf("bundle") !== -1) return { os: "linux", role: "bundle", sort: 1 };
      if (lower.indexOf("installer") !== -1) return { os: "linux", role: "installer", sort: 2 };
      return { os: "linux", role: "app", sort: 3 };
    }
    return { os: "other", role: "file", sort: 50 };
  }

  function roleLabel(lang, role) {
    var R = {
      bundle: {
        es: "Paquete sin conexión (recomendado para empezar)",
        en: "Offline bundle (recommended to start)",
      },
      installer: {
        es: "Instalador gráfico (medio portátil)",
        en: "Graphical installer (portable media)",
      },
      app: {
        es: "Aplicación",
        en: "Application",
      },
      checksums: {
        es: "Suma de comprobación SHA256",
        en: "SHA256 checksums",
      },
      file: {
        es: "Archivo",
        en: "File",
      },
    };
    var row = R[role] || R.file;
    return row[lang] || row.en;
  }

  function osBlock(lang, os) {
    var B = {
      win: {
        es: { title: "Windows (64 bits)", sub: "Ejecutables, instalador y paquete sin conexión" },
        en: { title: "Windows (64-bit)", sub: "Executables, installer, and offline bundle" },
      },
      linux: {
        es: { title: "Linux (64 bits)", sub: "Binario, instalador y paquete sin conexión" },
        en: { title: "Linux (64-bit)", sub: "Binary, installer, and offline bundle" },
      },
      meta: {
        es: { title: "Verificación de integridad", sub: "Comprueba las descargas con este fichero" },
        en: { title: "Integrity verification", sub: "Verify downloads using this file" },
      },
      other: {
        es: { title: "Otros archivos", sub: "Archivos adicionales del release" },
        en: { title: "Other files", sub: "Additional release files" },
      },
    };
    var row = B[os] || B.other;
    return row[lang] || row.en;
  }

  function iconSvg(os) {
    var common =
      ' xmlns="http://www.w3.org/2000/svg" width="22" height="22" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" focusable="false"';
    if (os === "win") {
      return (
        "<svg" +
        common +
        ' viewBox="0 0 24 24"><path d="M4 5h7v9H4V5zM13 5h7v4h-7V5zM13 11h7v8h-7v-8zM4 16h7v3H4v-3z"/></svg>'
      );
    }
    if (os === "linux") {
      return (
        "<svg" +
        common +
        ' viewBox="0 0 24 24"><rect x="3" y="4" width="18" height="14" rx="2"/><path d="M7 8h10M7 12h6M7 16h4"/></svg>'
      );
    }
    if (os === "meta") {
      return (
        "<svg" +
        common +
        ' viewBox="0 0 24 24"><path d="M12 3l7 4v10l-7 4-7-4V7l7-4z"/><path d="M9 12l2 2 4-4"/></svg>'
      );
    }
    return (
      "<svg" +
      common +
      ' viewBox="0 0 24 24"><path d="M13 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V9z"/><path d="M13 2v7h7"/></svg>'
    );
  }

  function renderCard(asset, lang) {
    var article = document.createElement("article");
    article.className = "download-card";

    var title = document.createElement("h3");
    title.className = "download-card__title";
    title.textContent = roleLabel(lang, asset._c.role);

    var fname = document.createElement("p");
    fname.className = "download-card__file";
    fname.textContent = asset.name;

    var link = document.createElement("a");
    link.className = "button-primary";
    link.href = asset.browser_download_url;
    link.rel = "noopener noreferrer";
    link.textContent = t("download");

    article.appendChild(title);
    article.appendChild(fname);
    article.appendChild(link);
    return article;
  }

  function renderGroup(osKey, items, container, lang) {
    if (!items.length) return;
    items.sort(function (a, b) {
      return a._c.sort - b._c.sort || a.name.localeCompare(b.name);
    });

    var section = document.createElement("section");
    section.className = "download-group download-group--" + osKey;

    var head = document.createElement("div");
    head.className = "download-group__head";

    var iconWrap = document.createElement("div");
    iconWrap.className = "download-group__icon";
    iconWrap.innerHTML = iconSvg(osKey === "other" ? "file" : osKey);

    var titles = document.createElement("div");
    var block = osBlock(lang, osKey);
    var h = document.createElement("h3");
    h.className = "download-group__title";
    h.textContent = block.title;
    var sub = document.createElement("p");
    sub.className = "download-group__sub";
    sub.textContent = block.sub;
    titles.appendChild(h);
    titles.appendChild(sub);

    head.appendChild(iconWrap);
    head.appendChild(titles);
    section.appendChild(head);

    var grid = document.createElement("div");
    grid.className = "download-grid";
    for (var i = 0; i < items.length; i++) {
      grid.appendChild(renderCard(items[i], lang));
    }
    section.appendChild(grid);
    container.appendChild(section);
  }

  function run() {
    var status = document.getElementById("download-status");
    var container = document.getElementById("download-assets");
    var versionEl = document.getElementById("release-version");
    var fallback = document.getElementById("download-fallback");
    var lang = getLang();

    if (!status || !container) return;

    status.hidden = false;
    status.className = "lp-banner lp-banner--loading";
    status.textContent = t("loading");
    if (fallback) {
      fallback.hidden = true;
    }

    fetch(API, { headers: { Accept: "application/vnd.github+json" } })
      .then(function (res) {
        if (!res.ok) throw new Error("HTTP " + res.status);
        return res.json();
      })
      .then(function (data) {
        var tag = data.tag_name || "";
        if (versionEl) versionEl.textContent = tag || "—";

        var assets = (data.assets || []).filter(function (x) {
          return x && x.name && x.browser_download_url;
        });
        if (!assets.length) {
          status.className = "lp-banner lp-banner--warn";
          status.textContent = t("empty");
          return;
        }

        var grouped = { win: [], linux: [], meta: [], other: [] };
        for (var j = 0; j < assets.length; j++) {
          var asset = assets[j];
          var c = classify(asset.name);
          asset._c = c;
          var key = c.os;
          if (!grouped[key]) grouped.other.push(asset);
          else grouped[key].push(asset);
        }

        status.hidden = true;
        container.innerHTML = "";
        renderGroup("win", grouped.win, container, lang);
        renderGroup("linux", grouped.linux, container, lang);
        renderGroup("meta", grouped.meta, container, lang);
        renderGroup("other", grouped.other, container, lang);
      })
      .catch(function () {
        status.hidden = true;
        if (versionEl) versionEl.textContent = "—";
        if (fallback) {
          fallback.textContent = t("fallback");
          fallback.hidden = false;
        }
      });
  }

  window.forusterReloadDownloads = run;

  function boot() {
    initLang();
    var sw = document.getElementById("lp-lang-switch");
    if (sw) sw.setAttribute("aria-label", getLang() === "es" ? "Idioma" : "Language");
    run();
  }

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", boot);
  } else {
    boot();
  }
})();
