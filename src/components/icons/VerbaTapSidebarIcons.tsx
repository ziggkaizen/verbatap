import React from "react";
import ThemedSidebarIcon from "./ThemedSidebarIcon";

import generalLight from "@/assets/sidebar-icons/light/general-24.png";
import generalDark from "@/assets/sidebar-icons/dark/general-24.png";
import historyLight from "@/assets/sidebar-icons/light/history-24.png";
import historyDark from "@/assets/sidebar-icons/dark/history-24.png";
import modelsLight from "@/assets/sidebar-icons/light/models-24.png";
import modelsDark from "@/assets/sidebar-icons/dark/models-24.png";
import advancedLight from "@/assets/sidebar-icons/light/advanced-24.png";
import advancedDark from "@/assets/sidebar-icons/dark/advanced-24.png";
import aboutLight from "@/assets/sidebar-icons/light/about-24.png";
import aboutDark from "@/assets/sidebar-icons/dark/about-24.png";

interface SidebarIconProps {
  size?: number | string;
  className?: string;
}

export const GeneralSidebarIcon: React.FC<SidebarIconProps> = (props) => (
  <ThemedSidebarIcon {...props} lightSrc={generalLight} darkSrc={generalDark} />
);

export const HistorySidebarIcon: React.FC<SidebarIconProps> = (props) => (
  <ThemedSidebarIcon {...props} lightSrc={historyLight} darkSrc={historyDark} />
);

export const ModelsSidebarIcon: React.FC<SidebarIconProps> = (props) => (
  <ThemedSidebarIcon {...props} lightSrc={modelsLight} darkSrc={modelsDark} />
);

export const AdvancedSidebarIcon: React.FC<SidebarIconProps> = (props) => (
  <ThemedSidebarIcon {...props} lightSrc={advancedLight} darkSrc={advancedDark} />
);

export const AboutSidebarIcon: React.FC<SidebarIconProps> = (props) => (
  <ThemedSidebarIcon {...props} lightSrc={aboutLight} darkSrc={aboutDark} />
);