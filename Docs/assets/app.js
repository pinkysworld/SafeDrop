/* SafeDrop - Site JavaScript */

function initNav() {
  var toggle = document.querySelector('.nav-toggle');
  var links = document.querySelector('.nav-links');
  if (!toggle || !links) return;

  toggle.addEventListener('click', function () {
    toggle.classList.toggle('open');
    links.classList.toggle('open');
    document.body.style.overflow = links.classList.contains('open') ? 'hidden' : '';
  });

  var dropdowns = links.querySelectorAll('.nav-dropdown');
  dropdowns.forEach(function (dd) {
    var trigger = dd.querySelector('.nav-dropdown-trigger');
    if (!trigger) return;
    trigger.addEventListener('click', function (e) {
      if (window.innerWidth > 768) return;
      e.preventDefault();
      dd.classList.toggle('open');
    });
  });

  links.querySelectorAll('a[href]').forEach(function (a) {
    a.addEventListener('click', function () {
      if (window.innerWidth <= 768) {
        toggle.classList.remove('open');
        links.classList.remove('open');
        document.body.style.overflow = '';
      }
    });
  });

  var page = document.body.dataset.page;
  if (page) {
    links.querySelectorAll('a').forEach(function (a) {
      var href = a.getAttribute('href');
      if (href === page + '.html' || (page === 'index' && href === 'index.html')) {
        a.classList.add('active');
      }
    });
  }
}

function initReveal() {
  var els = document.querySelectorAll('.reveal:not(.visible)');
  if (!els.length) return;
  var observer = new IntersectionObserver(function (entries) {
    entries.forEach(function (entry) {
      if (entry.isIntersecting) {
        entry.target.classList.add('visible');
        observer.unobserve(entry.target);
      }
    });
  }, { threshold: 0.08, rootMargin: '0px 0px -40px 0px' });
  els.forEach(function (el, i) {
    el.style.transitionDelay = Math.min(i * 0.05, 0.3) + 's';
    observer.observe(el);
  });
}

function cardForTrack(track) {
  return '<article class="track-card reveal">' +
    '<div class="track-meta">' +
    '<span class="meta-pill track-id">' + track.id + '</span>' +
    '<span class="meta-pill">' + track.category + '</span>' +
    '<span class="meta-pill">' + track.priority + '</span>' +
    '<span class="meta-pill">' + track.difficulty + '</span>' +
    '</div>' +
    '<h3>' + track.title + '</h3>' +
    '<p>' + track.summary + '</p>' +
    '<ul>' +
    '<li><strong>Methods:</strong> ' + track.methods + '</li>' +
    '<li><strong>Metrics:</strong> ' + track.metrics + '</li>' +
    '<li><strong>First wedge:</strong> ' + track.wedge + '</li>' +
    '</ul></article>';
}

function renderFeaturedTracks() {
  var target = document.querySelector('[data-featured-tracks]');
  if (!target || !window.SAFEDROP_TRACKS) return;
  var wanted = ['R06', 'R07', 'R02', 'R11', 'R12', 'R44', 'R47', 'R50'];
  var items = wanted.map(function (id) {
    return window.SAFEDROP_TRACKS.find(function (t) { return t.id === id; });
  }).filter(Boolean);
  target.innerHTML = items.map(cardForTrack).join('');
}

function renderBundles() {
  var target = document.querySelector('[data-bundle-grid]');
  if (!target || !window.SAFEDROP_BUNDLES) return;
  target.innerHTML = window.SAFEDROP_BUNDLES.map(function (bundle) {
    return '<article class="bundle-card reveal">' +
      '<div class="kicker">Research bundle</div>' +
      '<h3>' + bundle.name + '</h3>' +
      '<p>' + bundle.focus + '</p>' +
      '<div class="tag-list">' +
      bundle.tracks.map(function (t) { return '<span class="tag">' + t + '</span>'; }).join('') +
      '</div>' +
      '<p class="small-note">' + bundle.candidate_paper + '</p>' +
      '</article>';
  }).join('');
}

function renderResearchGrid() {
  var grid = document.querySelector('[data-research-grid]');
  if (!grid || !window.SAFEDROP_TRACKS) return;
  var search = document.querySelector('[data-track-search]');
  var filter = document.querySelector('[data-track-filter]');
  var count = document.querySelector('[data-track-count]');

  var render = function () {
    var q = (search ? search.value : '').toLowerCase();
    var f = filter ? filter.value : 'all';
    var items = window.SAFEDROP_TRACKS.filter(function (track) {
      var categoryHit = f === 'all' || track.category === f || track.priority === f;
      var text = (track.id + ' ' + track.title + ' ' + track.category + ' ' + track.bundle + ' ' +
        track.summary + ' ' + track.methods + ' ' + track.priority).toLowerCase();
      var queryHit = !q || text.indexOf(q) !== -1;
      return categoryHit && queryHit;
    });
    grid.innerHTML = items.map(cardForTrack).join('');
    if (count) count.textContent = items.length + ' tracks shown';
    initReveal();
  };
  if (search) search.addEventListener('input', render);
  if (filter) filter.addEventListener('change', render);
  render();
}

window.addEventListener('DOMContentLoaded', function () {
  initNav();
  renderFeaturedTracks();
  renderBundles();
  renderResearchGrid();
  requestAnimationFrame(function () { initReveal(); });
});
