// Load grid5.svg and add it to the page
fetch('grid5.svg')
    .then(response => response.text())
    .then(svgText => {
        // Parse the SVG text into a DOM element
        const parser = new DOMParser();
        const svgDoc = parser.parseFromString(svgText, 'image/svg+xml');
        const svgElement = svgDoc.documentElement;

        // Add the SVG to the body
        document.body.appendChild(svgElement);
    })
    .catch(error => {
        console.error('Error loading SVG:', error);
    });
