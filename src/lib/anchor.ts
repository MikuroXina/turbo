export type HorizontalAnchor = "LEFT" | "CENTER" | "RIGHT";

export type VerticalAnchor = "TOP" | "MIDDLE" | "BOTTOM";

export interface Anchor {
    horizontal: HorizontalAnchor;
    vertical: VerticalAnchor;
}
