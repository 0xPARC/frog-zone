import type React from "react";
import { useEffect, useRef } from "react";
import useStore, { type ActionLog } from "../store";

const actionLogStyles = {
	container: {
		background: "black",
		color: "limegreen",
		height: "100vh",
		width: "340px",
		fontFamily: "monospace",
		position: "absolute" as const,
		top: "0",
		right: "0",
		display: "flex",
		flexDirection: "column" as const,
		padding: "0 15px",
	},
	log: {
		overflowY: "scroll" as const,
		flex: 1,
		maxHeight: "calc(100% - 45px)",
	},
	message: {
		marginBottom: "5px",
		wordBreak: "break-word" as const,
		fontSize: "12px",
	},
	header: {
		padding: "20px 0px 10px 0px",
		maxHeight: "45px",
		fontWeight: "bold",
		color: "#728b83",
	},
};

const ACTION_LOG_PREFIX = ">";

export const TerminalActionLog: React.FC = () => {
	const logs = useStore((state) => state.actionLogs);
	const logEndRef = useRef<HTMLDivElement | null>(null);
	const scrollToBottom = () => {
		if (logEndRef.current) {
			logEndRef.current.scrollIntoView({ behavior: "smooth" });
		}
	};

	useEffect(() => {
		scrollToBottom();
	}, [logs]);

	return (
		<div style={actionLogStyles.container}>
			<div style={actionLogStyles.header}>FROG ZONE console:</div>
			<div style={actionLogStyles.log}>
				{logs.length > 0 &&
					logs.map((log: ActionLog, index) => (
						<div
							key={index}
							style={{
								...actionLogStyles.message,
								color: log?.color,
								fontWeight: index === logs.length - 1 ? "bold" : "normal",
							}}
						>
							{ACTION_LOG_PREFIX} {log?.message}
						</div>
					))}
				{/* Empty div to anchor the scroll position at the bottom */}
				<div ref={logEndRef} />
			</div>
		</div>
	);
};
