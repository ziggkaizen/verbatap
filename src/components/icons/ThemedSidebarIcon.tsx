import React from "react";

interface ThemedSidebarIconProps {
  lightSrc: string;
  darkSrc: string;
  size?: number | string;
  className?: string;
  alt?: string;
}

const ThemedSidebarIcon: React.FC<ThemedSidebarIconProps> = ({
  lightSrc,
  darkSrc,
  size = 20,
  className,
  alt = "",
}) => (
  <span
    className={className}
    aria-hidden={alt ? undefined : true}
    style={{
      display: "inline-grid",
      width: size,
      height: size,
      minWidth: size,
      minHeight: size,
      flexShrink: 0,
      alignItems: "center",
      justifyItems: "center",
      lineHeight: 0,
    }}
  >
    <img
      src={lightSrc}
      alt={alt}
      className="verbatap-brand-for-light"
      style={{
        gridArea: "1 / 1",
        width: "100%",
        height: "100%",
        objectFit: "contain",
      }}
    />
    <img
      src={darkSrc}
      alt=""
      aria-hidden="true"
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

export default ThemedSidebarIcon;
