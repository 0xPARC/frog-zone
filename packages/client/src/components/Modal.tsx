import React, { useEffect } from "react";

interface ModalProps {
	isOpen: boolean;
	onClose: () => void;
	title?: string;
	children: React.ReactNode;
	isCloseable?: boolean;
}

const modalStyles = {
	overlay: {
		position: "fixed" as "fixed",
		top: 0,
		left: 0,
		zIndex: 1000,
		width: "100vw",
		height: "100vh",
		background: "rgba(0, 0, 0, 0.5)",
		display: "flex",
		alignItems: "center",
		justifyContent: "center",
	},
	modal: {
		background: "black",
		padding: "20px",
		borderRadius: "5px",
		border: "1px solid white",
		width: "400px",
		maxWidth: "80%",
	},
	modalHeader: {
		display: "flex",
		justifyContent: "space-between",
		alignItems: "center",
	},
};

export const Modal: React.FC<ModalProps> = ({
	isOpen,
	onClose,
	title,
	children,
	isCloseable = true,
}) => {
	if (!isOpen) return null;

	// Close the modal when Escape key is pressed
	useEffect(() => {
		const handleEsc = (event: KeyboardEvent) => {
			if (event.key === "Escape") {
				onClose();
			}
		};
		if (isCloseable) {
			window.addEventListener("keydown", handleEsc);
		}
		return () => window.removeEventListener("keydown", handleEsc);
	}, [onClose, isCloseable]);

	return (
		<div style={modalStyles.overlay}>
			<div style={modalStyles.modal}>
				<div style={modalStyles.modalHeader}>
					<h2>{title}</h2>
				</div>
				<div>{children}</div>
			</div>
		</div>
	);
};
