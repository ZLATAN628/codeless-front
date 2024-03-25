/** @type {import('tailwindcss').Config} */
module.exports = {
	content: ['./src/**/*.{html,rs}', 'index.html'],

	theme: {
		extend: {
			gridTemplateColumns: {
				'auto-fit-min': 'repeat(auto-fit, minmax(100px, auto))',
        		'auto-fill-min': 'repeat(auto-fill, minmax(100px, auto))',
			}
		},
	},
	plugins: [
		require('@tailwindcss/typography'),
		require('daisyui'),
		function({ addUtilities }) {
			const newUtilities = {
			  '.scrollbar-hide': {
				'-webkit-scrollbar': {
				  display: 'none',
				},
				'scrollbar-width': 'none',
				'-ms-overflow-style': 'none',
			  },
			};
			addUtilities(newUtilities, ['responsive']);
		  }
	  
	],
	daisyui: {
        themes: ["light", "dark", "cupcake"],
    },
};
