import {
    Box,
    ButtonGroup,
    Flex,
    IconButton,
    Slider,
    SliderFilledTrack,
    SliderThumb,
    SliderTrack,
    Tooltip,
} from "@chakra-ui/react";
import { MdOutlinePlayArrow, MdOutlineSkipNext, MdOutlineSkipPrevious } from "react-icons/md";

import { useState } from "react";

export const SeekBar = () => {
    const [sliderValue, setSliderValue] = useState(0);
    const [showTooltip, setShowTooltip] = useState(false);

    return (
        <Slider
            aria-label="シークバー"
            defaultValue={0}
            value={sliderValue}
            min={0}
            max={1000}
            minH="100%"
            onChange={(v) => setSliderValue(v)}
            onMouseEnter={() => setShowTooltip(true)}
            onMouseLeave={() => setShowTooltip(false)}
        >
            <SliderTrack>
                <SliderFilledTrack />
            </SliderTrack>
            <Tooltip
                hasArrow
                color="white"
                placement="top"
                isOpen={showTooltip}
                label={`${(0).toString().padStart(2, "0")}:${(0).toFixed(3)}`}
            >
                <SliderThumb />
            </Tooltip>
        </Slider>
    );
};

export const Controls = () => {
    return (
        <Flex maxW="100%">
            <ButtonGroup>
                <IconButton aria-label="最初に戻る" icon={<MdOutlineSkipPrevious />} />
                <IconButton aria-label="再生と一時停止" icon={<MdOutlinePlayArrow />} />
                <IconButton aria-label="最後へ進む" icon={<MdOutlineSkipNext />} />
            </ButtonGroup>
            <Box flex="1 1" pl={4} pr={4}>
                <SeekBar />
            </Box>
        </Flex>
    );
};
