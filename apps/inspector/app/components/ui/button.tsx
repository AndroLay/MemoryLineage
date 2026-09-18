import type { ButtonHTMLAttributes } from "react";

type ButtonProps = ButtonHTMLAttributes<HTMLButtonElement> & {
  variant?: "primary" | "quiet" | "danger";
  size?: "default" | "small";
};

export function Button({ className = "", variant = "quiet", size = "default", ...props }: ButtonProps) {
  return <button className={`button button-${variant} button-${size} ${className}`} {...props} />;
}
