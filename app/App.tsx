import {
  Button,
  Flex,
  Heading,
  HStack,
  Text,
} from '@chakra-ui/react';
import { invoke } from '@tauri-apps/api/core';
import { useState } from 'react';

export default function App() {
  const [count, setCount] = useState(0);

  const call = async (cmd: string) => {
    const next = await invoke<number>(cmd);
    setCount(next);
  };

  return (
    <Flex
      align="center"
      bg="gray.900"
      color="white"
      direction="column"
      gap={8}
      h="100vh"
      justify="center"
    >
      <Heading size="2xl">Tauri Counter</Heading>
      <Text
        color="orange.400"
        fontSize="8xl"
        fontWeight="bold"
      >
        {count}
      </Text>
      <HStack gap={4}>
        <Button
          onClick={() => call('decrement')}
          size="lg"
          variant="ghost"
        >
          −
        </Button>
        <Button
          onClick={() => call('reset')}
          size="lg"
          variant="solid"
        >
          Reset
        </Button>
        <Button
          onClick={() => call('increment')}
          size="lg"
          variant="solid"
        >
          +
        </Button>
        <Button
          onClick={() => call('random_increment')}
          size="lg"
          variant="outline"
        >
          Random
        </Button>
      </HStack>
    </Flex>
  );
}
