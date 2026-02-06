export type ClientMessage =
  | {
      type: 'JoinGame';
    }
  | {
      type: 'PlayerMove';
      data: {
        src_square: string;
        dest_square: string;
      };
    };
