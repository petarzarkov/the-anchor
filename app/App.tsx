import {
  Box,
  Button,
  Flex,
  Input,
  Text,
} from '@chakra-ui/react';
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { useEffect, useState } from 'react';

export default function App() {
  const [task, setTask] = useState('');
  const [isEditing, setIsEditing] = useState(true);

  useEffect(() => {
    invoke<string>('get_task').then(saved => {
      if (saved) {
        setTask(saved);
        setIsEditing(false);
      }
    });
  }, []);

  const handleSave = async () => {
    if (!task.trim()) return;
    await invoke('set_task', { task });
    setIsEditing(false);
  };

  const handleClear = async () => {
    await invoke('clear_task');
    setTask('');
    setIsEditing(true);
  };

  const handleQuit = () => getCurrentWindow().close();

  return (
    <Box
      bg="rgba(13, 13, 13, 0.88)"
      borderRadius="lg"
      data-tauri-drag-region
      h="100vh"
      px={3}
      w="100vw"
    >
      {isEditing ? (
        <Flex align="center" gap={2} h="100%">
          <Input
            autoFocus
            color="white"
            flex={1}
            onChange={e => setTask(e.target.value)}
            onKeyDown={e => {
              if (e.key === 'Enter') handleSave();
              if (e.key === 'Escape' && task) {
                setIsEditing(false);
              }
            }}
            placeholder="What is your ONE task?"
            size="sm"
            value={task}
            variant="flushed"
          />
          <Button
            color="green.300"
            onClick={handleSave}
            size="xs"
            variant="ghost"
          >
            ✓
          </Button>
          <Button
            color="gray.500"
            onClick={handleQuit}
            size="xs"
            title="Quit"
            variant="ghost"
          >
            ×
          </Button>
        </Flex>
      ) : (
        <Flex
          align="center"
          data-tauri-drag-region
          gap={2}
          h="100%"
          justify="space-between"
        >
          <Text
            color="orange.300"
            data-tauri-drag-region
            flex={1}
            fontWeight="bold"
            overflow="hidden"
            textOverflow="ellipsis"
            whiteSpace="nowrap"
          >
            ⚓ {task}
          </Text>
          <Flex gap={1} shrink={0}>
            <Button
              color="gray.300"
              onClick={() => setIsEditing(true)}
              size="xs"
              title="Edit task"
              variant="ghost"
            >
              ✎
            </Button>
            <Button
              color="green.300"
              onClick={handleClear}
              size="xs"
              title="Mark complete"
              variant="ghost"
            >
              ✓
            </Button>
            <Button
              color="gray.500"
              onClick={handleQuit}
              size="xs"
              title="Quit"
              variant="ghost"
            >
              ×
            </Button>
          </Flex>
        </Flex>
      )}
    </Box>
  );
}
