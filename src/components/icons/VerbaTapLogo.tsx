import React from "react";
import wordmarkDark from "@/assets/branding/verbatap-wordmark-dark.png";
import wordmarkLight from "@/assets/branding/verbatap-wordmark-light.png";

interface VerbaTapLogoProps {
  width?: number | string;
  height?: number | string;
  className?: string;
}

const VerbaTapLogo: React.FC<VerbaTapLogoProps> = ({
  width = 180,
  height,
  className,
}) => (
  <span
    className={className}
    role="img"
    aria-label="VerbaTap"
    style={{
      display: "inline-grid",
      width,
      height,
      flexShrink: 0,
    }}
  >
    <img
      src={wordmarkDark}
      alt=""
      aria-hidden="true"
      className="verbatap-brand-for-light"
      style={{
        gridArea: "1 / 1",
        width: "100%",
        height: height ? "100%" : "auto",
        objectFit: "contain",
      }}
    />
    <img
      src={wordmarkLight}
      alt=""
      aria-hidden="true"
      className="verbatap-brand-for-dark"
      style={{
        gridArea: "1 / 1",
        width: "100%",
        height: height ? "100%" : "auto",
        objectFit: "contain",
      }}
    />
  </span>
);

export default VerbaTapLogo;
