import React from "react";
import markDark from "@/assets/branding/verbatap-mark-dark.png";
import markLight from "@/assets/branding/verbatap-mark-light.png";

interface VerbaTapMarkProps {
  width?: number | string;
  height?: number | string;
  size?: number | string;
  className?: string;
}

const VerbaTapMark: React.FC<VerbaTapMarkProps> = ({
  width,
  height,
  size,
  className,
}) => {
  const resolvedWidth = width ?? size ?? 64;
  const resolvedHeight = height ?? size ?? 64;

  return (
    <span
      className={className}
      aria-hidden="true"
      style={{
        display: "inline-grid",
        width: resolvedWidth,
        height: resolvedHeight,
        flexShrink: 0,
      }}
    >
      <img
        src={markDark}
        alt=""
        className="verbatap-brand-for-light"
        style={{
          gridArea: "1 / 1",
          width: "100%",
          height: "100%",
          objectFit: "contain",
        }}
      />
      <img
        src={markLight}
        alt=""
        className="verbatap-brand-for-dark"
        style={{
          gridArea: "1 / 1",
          width: "100%",
          height: "100%",
          objectFit: "contain",
        }}
      />
    </span>
  );
};

export default VerbaTapMark;
