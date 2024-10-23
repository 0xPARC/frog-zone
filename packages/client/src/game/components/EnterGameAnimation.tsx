import { AnimatePresence, motion } from "framer-motion";
import React, { useEffect, useState } from "react";
import { useReward } from "react-rewards";
import { getPlayerId } from "../../utils/getPlayerId";

const CONFETTI_ELEMENT_ID = "confettiReward";

export const EnterGameAnimation: React.FC = () => {
	const [showOverlay, setShowOverlay] = useState(true);
	const playerId = getPlayerId();
	const { reward: triggerConfetti } = useReward(
		CONFETTI_ELEMENT_ID,
		"confetti",
		{
			elementCount: 100,
			elementSize: 12,
			spread: 200,
			angle: -90,
			lifetime: 100,
		},
	);

	useEffect(() => {
		const timer = setTimeout(() => {
			setShowOverlay(false);
		}, 1000);

		const confettiTimer = setTimeout(() => {
			triggerConfetti();
		}, 1000);

		return () => {
			clearTimeout(timer);
			clearTimeout(confettiTimer);
		};
	}, []);

	return (
		<>
			<AnimatePresence>
				{showOverlay && (
					<motion.div
						style={overlayStyles}
						initial={{ opacity: 1 }}
						animate={{ opacity: 1 }}
						exit={{ opacity: 0 }}
						transition={{ duration: 1 }}
					>
						<h3>Get ready!</h3>
						<h1>YOU ARE FROG #{playerId}</h1>
					</motion.div>
				)}
			</AnimatePresence>
			<div
				id={CONFETTI_ELEMENT_ID}
				style={{
					position: "absolute",
					top: "-60px",
					left: "50%",
				}}
			/>
		</>
	);
};

const overlayStyles: React.CSSProperties = {
	position: "fixed",
	top: 0,
	left: 0,
	right: 0,
	bottom: 0,
	backgroundColor: "rgba(0, 0, 0, 0.8)",
	color: "white",
	display: "flex",
	flexDirection: "column",
	justifyContent: "center",
	alignItems: "center",
	zIndex: 1000,
};
