<script lang="ts">
	export let name: keyof typeof icons;
	export let width = '16px';
	export let height = '16px';
	export let focusable: string | number | null | undefined = undefined;
	let icons = {
		telemetry_icon: {
			box: 210,
			svg: `<rect x="14" y="42" width="51" height="10" fill="#FFAFBC"/>
<rect x="129" y="42" width="60" height="10" fill="#FFAFBC"/>
<rect x="89" y="42" width="20" height="10" fill="#F33742"/>
<rect x="109" y="90" width="41" height="10" fill="#FFAFBC"/>
<rect x="64" y="90" width="20" height="10" fill="#F33742"/>
<rect x="150" y="66" width="20" height="10" fill="#F33742"/>
<rect x="180" y="66" width="20" height="10" fill="#F33742"/>
<rect x="64" y="162" width="45" height="10" fill="#FFAFBC"/>
<rect x="29" y="172" width="45" height="10" fill="#FFAFBC"/>
<rect x="128" y="162" width="15" height="10" fill="#F33742"/>
<rect x="14" y="66" width="88" height="10" fill="#FFAFBC"/>
<rect x="84" y="114" width="52" height="10" fill="#FFAFBC"/>
<rect x="84" y="138" width="105" height="10" fill="#F33742"/>
<rect x="14" y="90" width="20" height="10" fill="#FFAFBC"/>`
		},
		virtual_network_icon: {
			box: 210,
			svg: `<path fill-rule="evenodd" clip-rule="evenodd" d="M96.7995 70.5953C92.8175 59.3878 90.2385 47.5163 89.2734 35.1923C94.3301 34.0583 99.5925 33.4592 105 33.4592C119.94 33.4592 133.801 38.0346 145.271 45.8619C142.898 55.9181 139.39 65.5365 134.889 74.5745C131.887 74.8545 128.845 74.9977 125.769 74.9977C115.677 74.9977 105.947 73.4562 96.7995 70.5953ZM145.87 72.9116C148.826 66.2408 151.287 59.3016 153.207 52.1406C154.862 53.651 156.446 55.2381 157.953 56.8962C160.141 54.6143 162.992 52.9732 166.196 52.2827C162.323 47.7904 157.964 43.7297 153.201 40.1818C139.746 30.161 123.059 24.2285 105 24.2285C97.4436 24.2285 90.1227 25.2673 83.1754 27.2127C74.997 29.5028 67.3454 33.0467 60.4402 37.6222C62.7383 39.6982 64.4367 42.4256 65.2443 45.5132C69.8874 42.4035 74.9135 39.8239 80.236 37.86C81.1339 47.6695 82.9908 57.2011 85.7121 66.3603C78.3457 63.0126 71.4719 58.7697 65.229 53.7697C64.405 56.8729 62.6804 59.6092 60.3521 61.6819C69.124 68.5544 79.0063 74.0774 89.6848 77.9369C94.3511 89.885 100.528 101.076 107.981 111.275C100.899 118.116 93.0651 124.182 84.6149 129.339C70.1747 110.747 60.5293 88.247 57.5317 63.6968C55.1923 65.0146 52.4915 65.7664 49.6152 65.7664C49.2344 65.7664 48.8567 65.7532 48.4825 65.7273C51.7507 91.1515 61.6915 114.484 76.4904 133.915C66.9004 138.884 56.6332 142.727 45.8639 145.269C38.0366 133.799 33.4612 119.938 33.4612 104.998C33.4612 90.2845 37.8997 76.6124 45.5154 65.2416C42.428 64.4339 39.7007 62.7356 37.6248 60.4376C29.1622 73.2089 24.2305 88.5295 24.2305 104.998C24.2305 123.057 30.163 139.744 40.1838 153.199C54.8991 172.956 78.4538 185.767 105 185.767C108.933 185.767 112.802 185.485 116.587 184.941C115.092 182.493 114.231 179.615 114.231 176.536C114.231 176.338 114.234 176.141 114.241 175.945C111.217 176.335 108.132 176.536 105 176.536C84.0647 176.536 65.2301 167.547 52.1424 153.205C62.7815 150.353 72.931 146.305 82.4382 141.215C92.045 152.248 103.315 161.794 115.857 169.462C117.267 166.572 119.514 164.165 122.283 162.557C110.416 155.529 99.7277 146.721 90.587 136.501C98.8957 131.321 106.634 125.313 113.687 118.592C125.684 133.021 140.377 145.127 156.99 154.138C152.717 158.657 147.862 162.619 142.546 165.904C144.583 168.231 145.962 171.149 146.394 174.367C154.641 169.436 161.933 163.077 167.929 155.633C179.087 141.783 185.769 124.165 185.769 104.998C185.769 95.9131 184.267 87.1733 181.498 79.0169C179.353 81.3456 176.531 83.0421 173.345 83.7951C175.42 90.492 176.538 97.6118 176.538 104.998C176.538 120.645 171.519 135.11 163.001 146.884C146.482 138.205 131.932 126.288 120.195 111.977C128.2 103.299 135.144 93.6277 140.822 83.1687C146.703 82.3337 152.429 81.0166 157.959 79.2586C155.795 77.0032 154.278 74.1208 153.71 70.911C151.14 71.6833 148.525 72.3518 145.87 72.9116ZM114.495 104.529C109.334 97.291 104.852 89.5359 101.143 81.3565C109.046 83.2343 117.292 84.2285 125.769 84.2285C127.057 84.2285 128.339 84.2055 129.616 84.16C125.219 91.4301 120.15 98.2484 114.495 104.529Z" fill="#1166E6"/>
<circle cx="49.8138" cy="49.8035" r="9.23077" fill="#0046C3"/>
<circle cx="169.659" cy="68.2436" r="9.23077" fill="#75A4FD"/>
<circle cx="130.405" cy="176.525" r="9.23077" fill="white"/>`
		},
		encypt_icon: {
			box: 210,
			svg: `<path fill-rule="evenodd" clip-rule="evenodd" d="M117.474 37.9765C97.3069 37.9765 80.9584 54.3249 80.9584 74.4918V80.2372H49.9214C35.3163 80.2372 23.4765 92.077 23.4765 106.682C23.4765 121.287 35.3163 133.127 49.9214 133.127H65.424C67.9028 133.127 69.9123 135.137 69.9123 137.615C69.9123 140.094 67.9028 142.104 65.424 142.104H49.9214C30.3587 142.104 14.5 126.245 14.5 106.682C14.5 87.1194 30.3587 71.2607 49.9214 71.2607H72.0949C73.7522 47.6441 93.4357 29 117.474 29C141.54 29 161.242 47.6882 162.858 71.3445C181.276 72.6053 195.825 87.9446 195.825 106.682C195.825 126.245 179.966 142.104 160.403 142.104H151.152C148.673 142.104 146.663 140.094 146.663 137.615C146.663 135.137 148.673 133.127 151.152 133.127H160.403C175.008 133.127 186.848 121.287 186.848 106.682C186.848 92.077 175.008 80.2372 160.403 80.2372H153.989V74.4918C153.989 54.3249 137.641 37.9765 117.474 37.9765Z" fill="#FBF8A2"/>
<path fill-rule="evenodd" clip-rule="evenodd" d="M131.258 112.88V122.156H133.438C139.882 122.156 145.107 127.38 145.107 133.825V166.34C145.107 174.768 138.275 181.6 129.847 181.6H86.76C78.3321 181.6 71.5 174.768 71.5 166.34V133.825C71.5 127.38 76.7246 122.156 83.1694 122.156H85.3494V112.88C85.3494 100.24 95.6526 90.04 108.304 90.04C120.954 90.04 131.258 100.24 131.258 112.88ZM83.1695 131.132C81.6822 131.132 80.4766 132.338 80.4766 133.825V166.34C80.4766 169.81 83.2898 172.624 86.7601 172.624H129.847C133.317 172.624 136.131 169.81 136.131 166.34V133.825C136.131 132.338 134.925 131.132 133.438 131.132H83.1695ZM94.3259 112.88C94.3259 105.25 100.558 99.0165 108.304 99.0165C116.049 99.0165 122.281 105.25 122.281 112.88V122.156H94.3259V112.88Z" fill="#FEDD00"/>
`
		},
		info_icon: {
			box: 20,
			svg: `<g clip-path="url(#clip0_3416_349)">
<path d="M3.33325 3.33398H16.6666V15.0007C16.6666 15.9211 15.9204 16.6673 14.9999 16.6673H4.99992C4.07944 16.6673 3.33325 15.9211 3.33325 15.0006V3.33398Z" stroke="#D4D5D7" stroke-width="1.66667" stroke-linecap="round" stroke-linejoin="round"/>
<rect x="10" y="6.66602" width="0.01" height="0.01" stroke="#D4D5D7" stroke-width="2.5" stroke-linejoin="round"/>
<path d="M10 10V13.3333" stroke="#D4D5D7" stroke-width="1.66667" stroke-linecap="round" stroke-linejoin="round"/>
</g>
<defs>
<clipPath id="clip0_3416_349">
<rect width="20" height="20" fill="white"/>
</clipPath>
</defs>`
		},
		back_icon: {
			box: 40,
			svg: `<g clip-path="url(#clip0_4725_2690)">
<path d="M24.6875 31.0156L13.6719 20" stroke="white" stroke-width="3.75" stroke-linecap="round" stroke-linejoin="round"/>
<path d="M13.6719 20L24.6875 8.98437" stroke="white" stroke-width="3.75" stroke-linecap="round" stroke-linejoin="round"/>
</g>
<defs>
<clipPath id="clip0_4725_2690">
<rect width="39.375" height="39.375" fill="white" transform="matrix(1 1.74846e-07 1.74846e-07 -1 0.3125 39.6875)"/>
</clipPath>
</defs>`
		},
		next_icon: {
			box: 40,
			svg: `<g clip-path="url(#clip0_4725_2695)">
<path d="M15.3125 31.0156L26.3281 20" stroke="white" stroke-width="3.75" stroke-linecap="round" stroke-linejoin="round"/>
<path d="M26.3281 20L15.3125 8.98437" stroke="white" stroke-width="3.75" stroke-linecap="round" stroke-linejoin="round"/>
</g>
<defs>
<clipPath id="clip0_4725_2695">
<rect width="39.375" height="39.375" fill="white" transform="translate(39.6875 39.6875) rotate(180)"/>
</clipPath>
</defs>`
		},
		settings_icon: {
			box: 32,
			svg: `<g clip-path="url(#clip0_2225_2247)">
<ellipse cx="16.0002" cy="15.9987" rx="2.66674" ry="2.66667" stroke="#FAFBFC" stroke-width="2.5"/>
<path d="M6.76572 7.51659C5.45143 8.93312 4.49708 10.6142 3.94139 12.4152L6.44288 13.8594C8.22072 14.8858 8.22074 17.4518 6.44292 18.4782L3.93897 19.9238C4.21493 20.8144 4.59372 21.6883 5.07958 22.5299C5.56542 23.3713 6.13288 24.1363 6.76619 24.8206L9.27018 23.3749C11.048 22.3485 13.2703 23.6316 13.2703 25.6844L13.2703 28.5729C15.1079 28.9922 17.0409 29.0062 18.9249 28.5763L18.9249 25.6844C18.9249 23.6316 21.1471 22.3486 22.9249 23.375L25.4294 24.821C26.7437 23.4044 27.6981 21.7234 28.2538 19.9224L25.7522 18.4781C23.9744 17.4517 23.9743 14.8857 25.7522 13.8593L28.2562 12.4137C27.9802 11.5231 27.6014 10.6492 27.1156 9.8077C26.6297 8.96619 26.0623 8.20118 25.4289 7.5169L22.925 8.96253C21.1471 9.98893 18.9249 8.7059 18.9249 6.65308L18.9249 3.76467C17.0873 3.3454 15.1542 3.33135 13.2703 3.76127L13.2703 6.65315C13.2703 8.70594 11.048 9.98892 9.27022 8.96252L6.76572 7.51659Z" stroke="#FAFBFC" stroke-width="3" stroke-linejoin="round"/>
</g>
<defs>
<clipPath id="clip0_2225_2247">
<rect width="32.0009" height="32" fill="white" transform="translate(-0.000244141)"/>
</clipPath>
</defs>`
		},
		refresh_icon: {
			box: 32,
			svg: `<path d="M18.9714 5.63749C24.6942 7.27848 28.0031 13.248 26.3621 18.9708C24.7212 24.6936 18.7516 28.0026 13.0288 26.3616C7.30601 24.7206 3.99705 18.7511 5.63803 13.0283C6.002 11.759 6.5789 10.6084 7.31882 9.60429M18.5003 9.00083L17.1865 4.87649L21.0003 3.50083" stroke="white" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"/>`
		},
		exit_icon: {
			box: 32,
			svg: `<g clip-path="url(#clip0_3356_579)">
<path d="M23.3327 22L28.666 15.9999M28.666 15.9999L23.3327 10M28.666 15.9999L12.666 15.9999" stroke="white" stroke-width="2.66667" stroke-linecap="round" stroke-linejoin="round"/>
<path d="M15.333 5.33594L8.66634 5.33594C7.19358 5.33594 5.99967 6.52984 5.99967 8.0026L5.99967 24.0026C5.99967 25.4754 7.19358 26.6693 8.66634 26.6693L15.333 26.6693" stroke="white" stroke-width="2.66667" stroke-linecap="round" stroke-linejoin="round"/>
</g>
<defs>
<clipPath id="clip0_3356_579">
<rect width="32" height="32" fill="white"/>
</clipPath>
</defs>`
		},
		active_status_icon: {
			box: 38,
			svg: `<g filter="url(#filter0_f_3308_322)">
<ellipse cx="18.4995" cy="19.0021" rx="7.875" ry="7.87511" stroke="#00AA13" stroke-width="2.66665" stroke-linecap="round" stroke-linejoin="round"/>
</g>
<ellipse cx="18.5" cy="19.002" rx="5.25" ry="5.25007" fill="#00AA13" stroke="#00AA13" stroke-width="2.66665" stroke-linecap="round" stroke-linejoin="round"/>
<defs>
<filter id="filter0_f_3308_322" x="0.14822" y="0.650173" width="36.7026" height="36.7036" filterUnits="userSpaceOnUse" color-interpolation-filters="sRGB">
<feFlood flood-opacity="0" result="BackgroundImageFix"/>
<feBlend mode="normal" in="SourceGraphic" in2="BackgroundImageFix" result="shape"/>
<feGaussianBlur stdDeviation="4.5714" result="effect1_foregroundBlur_3308_322"/>
</filter>
</defs>`
		},
		idle_status_icon: {
			box: 38,
			svg: `<g filter="url(#filter0_f_3308_322)">
<ellipse cx="18.4995" cy="19.0021" rx="7.875" ry="7.87511" stroke="#2E323C" stroke-width="2.66665" stroke-linecap="round" stroke-linejoin="round"/>
</g>
<ellipse cx="18.5" cy="19.002" rx="5.25" ry="5.25007" fill="#2E323C" stroke="#2E323C" stroke-width="2.66665" stroke-linecap="round" stroke-linejoin="round"/>
<defs>
<filter id="filter0_f_3308_322" x="0.14822" y="0.650173" width="36.7026" height="36.7036" filterUnits="userSpaceOnUse" color-interpolation-filters="sRGB">
<feFlood flood-opacity="0" result="BackgroundImageFix"/>
<feBlend mode="normal" in="SourceGraphic" in2="BackgroundImageFix" result="shape"/>
<feGaussianBlur stdDeviation="4.5714" result="effect1_foregroundBlur_3308_322"/>
</filter>
</defs>`
		},
		attention: {
			box: 50,
			svg: `<g clip-path="url(#clip0_6232_5119)">
<rect width="50" height="50" fill="#F33742"/>
<path d="M25 25L25 17.8889M41 25C41 33.8366 33.8366 41 25 41C16.1634 41 9 33.8366 9 25C9 16.1634 16.1634 9 25 9C33.8366 9 41 16.1634 41 25ZM25.0016 31.4029L25.0242 31.4029L25.0242 31.4256L25.0016 31.4256L25.0016 31.4029Z" stroke="white" stroke-width="3.75" stroke-linecap="square" stroke-linejoin="bevel"/>
</g>
<defs>
<clipPath id="clip0_6232_5119">
<rect width="50" height="50" fill="white"/>
</clipPath>
</defs>`
		},
		info: {
			box: 50,
			svg: `<g clip-path="url(#clip0_6232_5212)">
<rect width="50" height="50" fill="#0071EB"/>
<g clip-path="url(#clip1_6232_5212)">
<path d="M25 25V33.3333M9 9H41V41H9V9ZM25 17.666H25.0208V17.6868H25V17.666Z" stroke="white" stroke-width="3.75" stroke-linecap="square" stroke-linejoin="bevel"/>
</g>
</g>
<defs>
<clipPath id="clip0_6232_5212">
<rect width="50" height="50" fill="white"/>
</clipPath>
<clipPath id="clip1_6232_5212">
<rect width="50" height="50" fill="white"/>
</clipPath>
</defs>`
		},
		left_arrow: {
			box: 50,
			svg: `<path d="M28 35L18 25L28 15" stroke="white" stroke-width="3.5" stroke-linecap="square" stroke-linejoin="bevel"/>`
		},
		right_arrow: {
			box: 50,
			svg: `<path d="M18 35L28 25L18 15" stroke="white" stroke-width="3.5" stroke-linecap="square" stroke-linejoin="bevel"/>`
		},
		timeout_hourglass: {
			box: 50,
			svg: `<path d="M73.2739 103.925C73.2739 84.9218 61.048 68.7723 44.0274 62.9016M73.2739 103.925V112.292H14.7808V103.925M73.2739 103.925H55.309C53.3406 103.925 51.4312 103.252 49.8974 102.018L45.1185 98.1732C44.4825 97.6615 43.5751 97.6651 42.9432 98.1818L38.3036 101.975C36.7616 103.236 34.831 103.925 32.8391 103.925H14.7808M14.7808 103.925C14.7808 84.9218 27.0067 68.7723 44.0274 62.9016M44.0274 62.9016C33.1344 59.1445 24.2052 51.1773 19.1879 40.9466M44.0274 62.9016C54.9204 59.1445 63.8495 51.1773 68.8669 40.9466M19.1879 40.9466C16.3654 35.1914 14.7808 28.7199 14.7808 21.8786V13.5112H73.2739V21.8786C73.2739 28.7199 71.6893 35.1914 68.8669 40.9466M19.1879 40.9466H68.8669M44.0274 88.9696V88.3769M44.0274 77.5399V76.9472M44.0274 67.2626V66.67M79.2708 2.90137H8.72919C5.79947 2.90137 3.42447 5.27637 3.42447 8.20608V8.20608C3.42447 11.1358 5.79947 13.5108 8.72918 13.5108H79.2708C82.2005 13.5108 84.5755 11.1358 84.5755 8.20607V8.20607C84.5755 5.27636 82.2005 2.90137 79.2708 2.90137ZM8.78996 122.901H79.2649C82.1946 122.901 84.5696 120.526 84.5696 117.597V117.597C84.5696 114.667 82.1947 112.292 79.2649 112.292H8.78997C5.86026 112.292 3.48526 114.667 3.48526 117.597V117.597C3.48526 120.526 5.86025 122.901 8.78996 122.901Z" stroke="#848DA6" stroke-width="5" stroke-linecap="round"/>`
		},
		close_icon: {
			box: 20,
			svg: `<path d="M4.44417 16.0562L10 10.5004M15.5558 4.94451L10 10.5004M10 10.5004L4.44416 4.94451M10 10.5004L15.5558 16.0562" stroke="white" stroke-width="2" stroke-linecap="square" stroke-linejoin="bevel"/>`
		},
		spinner: {
			box: 30,
			svg: `<svg version="1.1" id="L9" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" x="0px" y="0px"
  viewBox="0 0 100 100" enable-background="new 0 0 0 0" xml:space="preserve">
    <path fill="#2D8AFF" d="M73,50c0-12.7-10.3-23-23-23S27,37.3,27,50 M30.9,50c0-10.5,8.5-19.1,19.1-19.1S69.1,39.5,69.1,50" strock-width="5">
      <animateTransform 
         attributeName="transform" 
         attributeType="XML" 
         type="rotate"
         dur="1s" 
         from="0 50 50"
         to="360 50 50" 
         repeatCount="indefinite" />
  </path>
</svg>`
		}
	} as const;
	let displayIcon = icons[name];
</script>

<svg
	fill="none"
	class={$$props.class}
	{focusable}
	{width}
	{height}
	viewBox="0 0 {displayIcon.box} {displayIcon.box}">{@html displayIcon.svg}</svg
>
