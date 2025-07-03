// Theme toggle functionality
class ThemeManager {
    constructor() {
        this.theme = localStorage.getItem('theme') || this.getPreferredTheme();
        this.themeToggle = document.getElementById('themeToggle');
        this.themeIcon = this.themeToggle.querySelector('.theme-icon');
        
        this.init();
    }

    init() {
        // Set initial theme
        this.setTheme(this.theme);
        
        // Add event listener for theme toggle
        this.themeToggle.addEventListener('click', () => {
            this.toggleTheme();
        });

        // Listen for system theme changes
        window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
            if (!localStorage.getItem('theme')) {
                this.setTheme(e.matches ? 'dark' : 'light');
            }
        });
    }

    getPreferredTheme() {
        return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
    }

    setTheme(theme) {
        this.theme = theme;
        document.documentElement.setAttribute('data-theme', theme);
        this.updateThemeIcon();
        
        // Save to localStorage
        localStorage.setItem('theme', theme);
    }

    toggleTheme() {
        const newTheme = this.theme === 'light' ? 'dark' : 'light';
        this.setTheme(newTheme);
    }

    updateThemeIcon() {
        this.themeIcon.textContent = this.theme === 'light' ? '🌙' : '☀️';
        this.themeToggle.setAttribute('aria-label', 
            `Switch to ${this.theme === 'light' ? 'dark' : 'light'} mode`
        );
    }
}

// Smooth scrolling for navigation links
class SmoothScroll {
    constructor() {
        this.init();
    }

    init() {
        document.querySelectorAll('a[href^="#"]').forEach(anchor => {
            anchor.addEventListener('click', (e) => {
                e.preventDefault();
                const target = document.querySelector(anchor.getAttribute('href'));
                if (target) {
                    const headerHeight = document.querySelector('.header').offsetHeight;
                    const targetPosition = target.offsetTop - headerHeight - 20;
                    
                    window.scrollTo({
                        top: targetPosition,
                        behavior: 'smooth'
                    });
                }
            });
        });
    }
}

// Image lazy loading and error handling
class ImageManager {
    constructor() {
        this.init();
    }

    init() {
        // Add loading state and error handling for images
        document.querySelectorAll('img').forEach(img => {
            img.addEventListener('load', () => {
                img.style.opacity = '1';
            });

            img.addEventListener('error', () => {
                img.style.opacity = '0.5';
                console.warn(`Failed to load image: ${img.src}`);
            });

            // Set initial opacity for smooth loading
            img.style.opacity = '0';
            img.style.transition = 'opacity 0.3s ease';
        });
    }
}

// Copy code functionality
class CodeManager {
    constructor() {
        this.init();
    }

    init() {
        // Add copy buttons to code blocks (optional enhancement)
        document.querySelectorAll('.code-block').forEach(block => {
            this.addCopyButton(block);
        });
    }

    addCopyButton(codeBlock) {
        const button = document.createElement('button');
        button.className = 'copy-button';
        button.innerHTML = '📋';
        button.title = 'Copy code';
        button.style.cssText = `
            position: absolute;
            top: 0.5rem;
            right: 0.5rem;
            background: var(--code-bg);
            border: 1px solid var(--border-color);
            border-radius: 0.25rem;
            padding: 0.25rem 0.5rem;
            cursor: pointer;
            opacity: 0;
            transition: opacity 0.2s ease;
            font-size: 0.8rem;
        `;

        codeBlock.style.position = 'relative';
        codeBlock.appendChild(button);

        codeBlock.addEventListener('mouseenter', () => {
            button.style.opacity = '1';
        });

        codeBlock.addEventListener('mouseleave', () => {
            button.style.opacity = '0';
        });

        button.addEventListener('click', () => {
            const code = codeBlock.querySelector('pre').textContent;
            navigator.clipboard.writeText(code).then(() => {
                button.innerHTML = '✓';
                setTimeout(() => {
                    button.innerHTML = '📋';
                }, 1000);
            }).catch(() => {
                console.warn('Failed to copy code to clipboard');
            });
        });
    }
}

// Performance monitoring
class PerformanceMonitor {
    constructor() {
        this.init();
    }

    init() {
        // Monitor Core Web Vitals
        this.observeLCP();
        this.observeFID();
        this.observeCLS();
    }

    observeLCP() {
        if ('PerformanceObserver' in window) {
            new PerformanceObserver((entryList) => {
                const entries = entryList.getEntries();
                const lastEntry = entries[entries.length - 1];
                console.log('LCP:', lastEntry.startTime);
            }).observe({ entryTypes: ['largest-contentful-paint'] });
        }
    }

    observeFID() {
        if ('PerformanceObserver' in window) {
            new PerformanceObserver((entryList) => {
                const entries = entryList.getEntries();
                entries.forEach(entry => {
                    console.log('FID:', entry.processingStart - entry.startTime);
                });
            }).observe({ entryTypes: ['first-input'] });
        }
    }

    observeCLS() {
        if ('PerformanceObserver' in window) {
            let clsValue = 0;
            new PerformanceObserver((entryList) => {
                for (const entry of entryList.getEntries()) {
                    if (!entry.hadRecentInput) {
                        clsValue += entry.value;
                    }
                }
                console.log('CLS:', clsValue);
            }).observe({ entryTypes: ['layout-shift'] });
        }
    }
}

// Initialize everything when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    new ThemeManager();
    new SmoothScroll();
    new ImageManager();
    new CodeManager();
    
    // Only monitor performance in development
    if (window.location.hostname === 'localhost') {
        new PerformanceMonitor();
    }
});

// Add loading indicator
window.addEventListener('load', () => {
    document.body.classList.add('loaded');
});