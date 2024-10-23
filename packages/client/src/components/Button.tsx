import React, { ButtonHTMLAttributes } from "react";

// Extend HTML button props for general-purpose use
interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
	loading?: boolean; // Optional prop for loading state
}

export const Button: React.FC<ButtonProps> = ({
	loading = false,
	children,
	...props
}) => {
	return (
		<button
			{...props}
			disabled={props.disabled || loading}
			style={{
				background: "blue",
				color: "white",
				fontFamily: "monospace",
				borderRadius: "5px",
				padding: "10px 15px",
				border: "none",
				cursor: props.disabled || loading ? "not-allowed" : "pointer",
				...props.style,
			}}
		>
			{loading ? "Updating..." : children}
		</button>
	);
};
