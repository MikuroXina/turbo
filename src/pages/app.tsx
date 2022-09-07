import { Box, Flex } from "@chakra-ui/react";

import { Controls } from "../components/controls";
import { Preview } from "../components/preview";
import { usePlugins } from "../lib/plugin";

export const App = () => {
    const plugins = usePlugins();
    for (const plugin of plugins) {
        if (plugin.metadata) {
            console.dir(plugin.metadata());
        }
    }

    return (
        <Flex direction="column" h="100%">
            <Box flex="1 1">
                <Preview />
            </Box>
            <Box flex="0 0" w="100%">
                <Controls />
            </Box>
        </Flex>
    );
};
