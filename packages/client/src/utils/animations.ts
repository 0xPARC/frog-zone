export const completedMoveAnimation = (image: Phaser.GameObjects.Image) => {
	const ANIMATION_DURATION = 800; // Run the animation for this long
	const INTERVAL_DURATION = 200; // Loop the animation at this interval
	const MIN_OPACITY = 0;
	const MAX_OPACITY = 1;
	let value = 0.5;
	const interval = setInterval(() => {
		value = value === MIN_OPACITY ? MAX_OPACITY : MIN_OPACITY;
		image.setAlpha(value);
	}, INTERVAL_DURATION);

	setTimeout(() => {
		clearInterval(interval);
		image.setAlpha(1);
	}, ANIMATION_DURATION);
};
