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
      if (href === page + '.html' || (page === 'index' && href =      if (href === page + '.html' |Li      if (href === page + '.html'         if (href === page + '.html' || (page === 'index' && href =      if (href === sible)');
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
    '<span class="meta-pill track-id">' + track.i    '</s   >' +
    '<s    '<s    '<s    '<s    '<s    '<s    y + '</span>' +
    '<span class="meta-pill">' + track.priority + '</span>' +
    '<span class="meta-pill">' + track.difficulty + '</span>' +
    '</div>' +
    '<h3>' + track.title + '</h3>' +
    '<p>' + track.summary + '</p>' +
    '<ul>' +
    '<li><strong>Methods:</strong> ' + track.methods + '</li>' +
    '<li><strong>Metrics:</strong> ' + track.metrics + '</li>' +
    '<li><strong>First wedge:</strong> ' + track.wedge + '    '<li><strong>First wedge:</strongnction renderFeaturedTracks() {
  va  va  va  va  va  va  va  va  va  va  va  va ur  va  va  va  va  va  va  va  va  va w.SAFEDROP_TRACKS) re  va  va  va  va  va  va  va  va  va  va  va  va ur  2', 'R44', 'R47', 'R50'];
  var items =   var items =   var items =   varturn window.SAFEDROP_TRACKS.find(function (t) { return t.id ===   var items =   var items n);
                                   ForTrack).join(                               ) {
  var target = document.querySelector('[data-bundle-grid]');
                                                              ne                                                            re                                                            ass=             ar                                   un                                   bundle.focus +                                                               map(function (t) { return '<span class="tag">' + t + '</span>'; }).join('') +
      '</div>' +
      '<p class="small-note">' + bundle.candidate_paper + '</p>' +
      '</article>';
  }).join('')  }).join('')  }).join('')  }).join('')  }).join('')  }).join('')  }).join('')  }).join('')  }).join('')  }).join('')  }).join('')  }).join('')  }).join('')  }).join('')  }).join('')  })da  }).join('')  }).join('')  }).join('')  }).join('')  }).join('')  }).join('')  }).join('')  }).join('')  }).join('')  }).join('')  }).join);  }).join('')  }).join('')  }).join('')  }).join('')  }).join('')  }).join('')  }).()  }).join('')  }).join('')  }).join('')  }).join('')  }).s = window.SAFEDROP_TRACKS.filter(function (track) {
      var categoryHit = f === 'all' || track.category === f || track.priority === f;
      var text = (track.id + ' ' + track.title + ' ' + track.category + ' ' + track.bundle + ' ' +
        track.summary + ' ' + track.methods + ' ' + track.priority).toLowerCase();
      var queryHit = !q || text.indexOf(q) !== -1;
      return categoryHit && queryHit;
    });
    grid.innerHTML = items.map(cardF    grid.innerHTML = items.map(cardF    grid.innerHTML = items.mah     grid.innerHTML
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
