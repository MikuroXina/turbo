import { Box, Center } from "@chakra-ui/react";

import { css } from "@emotion/css";

export const Preview = () => (
    <Box w="100%" h="100%">
        <Center h="100%">
            <canvas
                data-raw-handle={1}
                className={css`
                    background-color: black;
                    max-width: 100%;
                    max-height: 100%;
                `}
                width="1920"
                height="1080"
            ></canvas>
        </Center>
    </Box>
);
