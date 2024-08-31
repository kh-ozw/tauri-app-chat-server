type MsgPanelProps = {
    index: number;
    messageinfo: MessageInfo;
}

export interface MessageInfo {
    message: string;
    user: number;
    date: Date;
}

export const MsgPanel = (props: MsgPanelProps) => {
    const { index, messageinfo } = props;
    return (
        <div className="rounded-border" key={index}>
            <div>
                <div className="date">{messageinfo.date.toISOString().replace('T', ' ').substring(0, 19)}</div>
                <div className="user-name">{"ID: " + messageinfo.user}</div>
            </div>
            <div className="message">{`>   ${messageinfo.message}`}</div>
        </div>
    );
}